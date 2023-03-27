use std::{
    io::{BufRead, Write},
    path::PathBuf,
};

use base64::{engine::general_purpose, Engine as _};
use tokio::sync::mpsc;

use crate::{SearchOptions, SearchState};

use requests::*;
use responses::*;

mod requests;
mod responses;

pub async fn run_service(options: SearchOptions) {
    let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<ServiceRequest>();
    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<ServiceResponse>();

    std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();

        loop {
            let mut buf = vec![];
            stdin.read_until(b',', &mut buf).unwrap();
            buf.pop();

            let decoded = general_purpose::STANDARD
                .decode(buf)
                .expect("should be valid base64");

            match serde_json::from_slice::<ServiceRequest>(&decoded) {
                Ok(msg) => {
                    incoming_tx.send(msg).expect("Channel is open");
                }
                Err(_) => {}
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
                message_id,
                payload,
            }) {
                eprintln!("Internal error: Failed to respond to message {message_id}: {e}");
                std::process::exit(1);
            }
        };

        let err = |msg: &str| {
            send(ResponseAction::Error {
                message: msg.into(),
            })
        };

        match msg.payload {
            RequestAction::NewIndex => {
                let index_id = indexes.len();
                indexes.insert(index_id, SearchState::new(options.clone()));
                send(ResponseAction::CreatedIndex {
                    index_id: index_id as u32,
                });
            }
            RequestAction::AddFile {
                index_id,
                file_path,
                file_contents,
            } => {
                let index = indexes
                    .get_mut(index_id as usize)
                    .expect("Requested index should exist");
                let data = index
                    .fossick_synthetic_file(PathBuf::from(file_path), file_contents)
                    .await;
                match data {
                    Ok(data) => send(ResponseAction::AddedFile {
                        page_word_count: data.fragment.data.word_count as u32,
                        page_url: data.fragment.data.url.clone(),
                        page_meta: data.fragment.data.meta.clone(),
                    }),
                    Err(_) => err("Failed to add file"),
                }
            }
            RequestAction::WriteFiles { index_id } => {
                let mut index = indexes.remove(index_id as usize);
                index.build_indexes().await;
                index.write_files().await;
            }
        }
    }
}
