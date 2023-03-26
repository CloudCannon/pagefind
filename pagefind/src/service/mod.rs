use std::{
    cell::{Cell, RefCell},
    io::{BufRead, Read, Write},
    path::PathBuf,
    rc::Rc,
};

use base64::{engine::general_purpose, Engine as _};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::{PagefindInboundConfig, SearchOptions, SearchState};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceRequest {
    pub message_id: u32,
    pub payload: ServiceAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ServiceAction {
    NewIndex {
        id: u32,
    },
    AddFile {
        index_id: u32,
        file_path: String,
        file_contents: String,
    },
    WriteFiles {
        index_id: u32,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceResponse {
    pub message_id: u32,
    pub msg: String,
}

pub async fn run_service(options: SearchOptions) {
    let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<ServiceRequest>();
    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<ServiceResponse>();

    let o1 = outgoing_tx.clone();
    std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();

        loop {
            let mut buf = vec![];
            stdin.read_until(b',', &mut buf).unwrap();
            buf.pop();

            let decoded = general_purpose::STANDARD
                .decode(buf)
                .expect("should be valid base64");

            o1.send(ServiceResponse {
                message_id: 1234,
                msg: std::str::from_utf8(&decoded).unwrap().to_string(),
            })
            .unwrap();

            let msg: ServiceRequest =
                serde_json::from_slice(&decoded).expect("should be a valid json");

            incoming_tx.send(msg).expect("Channel is open");
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

    let mut indexes = HashMap::new();

    loop {
        let Some(msg) = incoming_rx.recv().await else {
            return;
        };

        if let Err(e) = outgoing_tx.send(ServiceResponse {
            message_id: msg.message_id,
            msg: format!("Handing {msg:#?}").into(),
        }) {
            eprintln!("{e:#?}");
        }

        match msg.payload {
            ServiceAction::NewIndex { id } => {
                indexes.insert(id, SearchState::new(options.clone()));
            }
            ServiceAction::AddFile {
                index_id,
                file_path,
                file_contents,
            } => {
                let index = indexes
                    .get_mut(&index_id)
                    .expect("Requested index should exist");
                index
                    .fossick_synthetic_file(PathBuf::from(file_path), file_contents)
                    .await;
            }
            ServiceAction::WriteFiles { index_id } => {
                let mut index = indexes
                    .remove(&index_id)
                    .expect("Requested index should exist");
                index.build_indexes().await;
                index.write_files().await;
            }
        }
    }
}
