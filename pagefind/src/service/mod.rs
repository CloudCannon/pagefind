use std::{
    io::{BufRead, Write},
    path::PathBuf,
};

use base64::{engine::general_purpose, Engine as _};
use hashbrown::HashMap;
use rust_patch::Patch;
use tokio::sync::mpsc;

use crate::{
    fossick::{parser::DomParserResult, Fossicker},
    PagefindInboundConfig, SearchOptions, SearchState,
};

use requests::*;
use responses::*;

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

            let Ok(decoded) = general_purpose::STANDARD
                .decode(buf) else {
                    parse_error_outgoing_tx
                        .send(ServiceResponse {
                            message_id: None,
                            payload: ResponseAction::Error {
                                original_message: None,
                                message: "Unparseable message, not valid base64".into()
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
            indexes: &'a mut Vec<Option<SearchState>>,
            index_id: u32,
            err: impl FnOnce(&str),
        ) -> Option<&'a mut SearchState> {
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

                match SearchOptions::load(service_options) {
                    Ok(opts) => {
                        indexes.insert(index_id, Some(SearchState::new(opts)));
                        send(ResponseAction::NewIndex {
                            index_id: index_id as u32,
                        });
                    }
                    Err(_) => {
                        err("Invalid config supplied");
                    }
                }
            }
            RequestAction::AddFile {
                index_id,
                file_path,
                file_contents,
            } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    let file = Fossicker::new_synthetic(PathBuf::from(file_path), file_contents);
                    let data = index.fossick_one(file).await;
                    match data {
                        Ok(data) => send(ResponseAction::IndexedFile {
                            page_word_count: data.fragment.data.word_count as u32,
                            page_url: data.fragment.data.url.clone(),
                            page_meta: data.fragment.data.meta.clone(),
                        }),
                        Err(_) => err("Failed to add file"),
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
                    let data = DomParserResult {
                        digest: content,
                        filters: filters.unwrap_or_default(),
                        sort: sort.unwrap_or_default(),
                        meta: meta.unwrap_or_default(),
                        anchor_content: HashMap::new(),
                        has_custom_body: false,
                        force_inclusion: true,
                        has_html_element: true,
                        language,
                    };
                    let file = Fossicker::new_with_data(url, data);
                    let data = index.fossick_one(file).await;
                    match data {
                        Ok(data) => send(ResponseAction::IndexedFile {
                            page_word_count: data.fragment.data.word_count as u32,
                            page_url: data.fragment.data.url.clone(),
                            page_meta: data.fragment.data.meta.clone(),
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
                    let defaults: PagefindInboundConfig =
                        serde_json::from_str("{}").expect("All fields have serde defaults");
                    let glob = glob.unwrap_or_else(|| defaults.glob);

                    let data = index.fossick_many(PathBuf::from(path), glob).await;
                    match data {
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
                bundle_path,
            } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    index.build_indexes().await;
                    let bundle_path = index.write_files(bundle_path.map(Into::into)).await;
                    send(ResponseAction::WriteFiles {
                        bundle_path: bundle_path.to_string_lossy().into(),
                    });
                }
            }
            RequestAction::GetFiles { index_id } => {
                if let Some(index) = get_index(&mut indexes, index_id, err) {
                    index.build_indexes().await;
                    let files = index.get_files().await;
                    send(ResponseAction::GetFiles {
                        files: files
                            .into_iter()
                            .map(|file| SyntheticFileResponse {
                                path: file.filename.to_string_lossy().into(),
                                content: general_purpose::STANDARD.encode(file.contents),
                            })
                            .collect(),
                    });
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
