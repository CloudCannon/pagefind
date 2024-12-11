use std::io::{BufRead, Write};

pub use api::PagefindIndex;
use base64::{engine::general_purpose, Engine as _};
use rust_patch::Patch;
use tokio::sync::mpsc;

pub mod api;

use requests::*;
use responses::*;

use crate::PagefindInboundConfig;

mod requests;
mod responses;

pub async fn run_service() {
    let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<ServiceRequest>();
    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<ServiceResponse>();

    let parse_error_outgoing_tx = outgoing_tx.clone();

    std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();

        loop {
            let mut buf = vec![];
            stdin.read_until(b',', &mut buf).unwrap();

            if buf.pop().is_none() {
                // EOF Reached
                std::process::exit(0);
            }

            let Ok(decoded) = general_purpose::STANDARD.decode(buf) else {
                parse_error_outgoing_tx
                    .send(ServiceResponse {
                        message_id: None,
                        payload: ResponseAction::Error {
                            original_message: None,
                            message: "Unparseable message, not valid base64".into(),
                        },
                    })
                    .expect("Channel is open");
                return;
            };

            match serde_json::from_slice::<ServiceRequest>(&decoded) {
                Ok(msg) => {
                    incoming_tx.send(msg).expect("Channel is open");
                }
                Err(e) => {
                    let error = match std::str::from_utf8(&decoded[..]) {
                        Ok(msg) => ResponseAction::Error {
                            original_message: Some(msg.to_string()),
                            message: format!("{e}"),
                        },
                        Err(_) => ResponseAction::Error {
                            original_message: None,
                            message: "Pagefind was unable to parse the message it was provided via the service".to_string(),
                        },
                    };

                    parse_error_outgoing_tx
                        .send(ServiceResponse {
                            message_id: None,
                            payload: error,
                        })
                        .expect("Channel is open");
                }
            }
        }
    });

    std::thread::spawn(move || {
        let mut stdout = std::io::stdout().lock();

        loop {
            let msg = outgoing_rx.blocking_recv();
            let encoded = general_purpose::STANDARD.encode(serde_json::to_vec(&msg).unwrap());

            stdout.write_all(encoded.as_bytes()).unwrap();
            stdout.write(b",").unwrap();
            stdout.flush().unwrap();
        }
    });

    let mut indexes = vec![];

    // TODO: Handle incoming messages concurrently
    loop {
        let Some(msg) = incoming_rx.recv().await else {
            return;
        };
        let message_id = msg.message_id;

        let send = |payload| {
            if let Err(e) = outgoing_tx.send(ServiceResponse {
                message_id: Some(message_id),
                payload,
            }) {
                eprintln!("Internal error: Failed to respond to message {message_id}: {e}");
                std::process::exit(1);
            }
        };

        let err = |msg: &str| {
            send(ResponseAction::Error {
                original_message: None,
                message: msg.into(),
            })
        };

        fn get_index<'a>(
            indexes: &'a mut Vec<Option<api::PagefindIndex>>,
            index_id: u32,
            err: impl FnOnce(&str),
        ) -> Option<&'a mut api::PagefindIndex> {
            match indexes.get_mut(index_id as usize) {
                Some(Some(index)) => Some(index),
                Some(None) => {
                    err("Index has been deleted from the Pagefind service and no longer exists");
                    None
                }
                None => {
                    err("Invalid index, does not yet exist in the Pagefind service");
                    None
                }
            }
        }

        match msg.payload {
            RequestAction::NewIndex { config } => {
                let index_id = indexes.len();
                let mut service_options: PagefindInboundConfig =
                    serde_json::from_str("{}").expect("All fields have serde defaults");

                service_options.service = true;
                if let Some(config) = config {
                    service_options = config.apply(service_options);
                }

                match PagefindIndex::new(service_options) {
                    Some(index) => {
                        indexes.insert(index_id, Some(index));
                        send(ResponseAction::NewIndex {
                            index_id: index_id as u32,
                        });
                    }
                    None => {
                        err("Invalid config supplied");
                    }
                }
            }
            RequestAction::AddFile {
                index_id,
                file_path,
                url,
                file_contents,
            } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    let page_fragment = index.add_file(file_path, url, file_contents).await;
                    match page_fragment {
                        Ok(data) => send(ResponseAction::IndexedFile {
                            page_word_count: data.page_word_count,
                            page_url: data.page_url.clone(),
                            page_meta: data.page_meta.clone(),
                        }),
                        Err(message) => err(&message),
                    }
                }
            }
            RequestAction::AddRecord {
                index_id,
                url,
                content,
                language,
                meta,
                filters,
                sort,
            } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    let data = index
                        .add_record(url, content, language, meta, filters, sort)
                        .await;
                    match data {
                        Ok(data) => send(ResponseAction::IndexedFile {
                            page_word_count: data.page_word_count,
                            page_url: data.page_url.clone(),
                            page_meta: data.page_meta.clone(),
                        }),
                        Err(_) => err("Failed to add file"),
                    }
                }
            }
            RequestAction::AddDir {
                index_id,
                path,
                glob,
            } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    match index.add_dir(path, glob).await {
                        Ok(page_count) => send(ResponseAction::IndexedDir {
                            page_count: page_count as u32,
                        }),
                        Err(_) => err("Failed to index directory"),
                    }
                }
            }
            RequestAction::BuildIndex { index_id } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    index.build_indexes().await;
                    send(ResponseAction::BuildIndex {});
                }
            }
            RequestAction::WriteFiles {
                index_id,
                output_path,
            } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    index.build_indexes().await;
                    let resolved_output_path = index.write_files(output_path.map(Into::into)).await;
                    send(ResponseAction::WriteFiles {
                        output_path: resolved_output_path,
                    });
                }
            }
            RequestAction::GetFiles { index_id } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    index.build_indexes().await;
                    let files = index.get_files().await;
                    send(ResponseAction::GetFiles { files });
                }
            }
            RequestAction::DeleteIndex { index_id } => match indexes.get_mut(index_id as usize) {
                Some(slot) => {
                    *slot = None;
                    send(ResponseAction::DeleteIndex {});
                }
                None => {
                    err("Index does not exist in the Pagefind service");
                }
            },
        }
    }
}
