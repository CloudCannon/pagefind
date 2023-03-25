use std::{
    cell::{Cell, RefCell},
    io::{BufRead, Read, Write},
    rc::Rc,
};

use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::PagefindInboundConfig;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceRequest {
    pub message_id: u32,
    pub payload: ServiceAction,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ServiceAction {
    NewIndex { id: u32 },
    Other { custom: String },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceResponse {
    pub message_id: u32,
}

pub async fn run_service(config: PagefindInboundConfig) {
    let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel::<ServiceRequest>();

    std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();

        loop {
            let mut buf = vec![];
            stdin.read_until(b',', &mut buf).unwrap();
            buf.pop();

            let msg: ServiceRequest = serde_json::from_slice(
                &general_purpose::STANDARD
                    .decode(buf)
                    .expect("should be valid base64"),
            )
            .expect("should be a valid json");

            incoming_tx.send(msg).expect("Channel is open");
        }
    });

    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel::<ServiceResponse>();

    std::thread::spawn(move || {
        let mut stdout = std::io::stdout().lock();

        let msg = outgoing_rx.blocking_recv();
        let encoded = general_purpose::STANDARD.encode(serde_json::to_vec(&msg).unwrap());

        stdout.write_all(encoded.as_bytes()).unwrap();
        stdout.write(b",").unwrap();
        stdout.flush().unwrap();
    });

    loop {
        let Some(msg) = incoming_rx.recv().await else {
            return;
        };

        match msg.payload {
            ServiceAction::NewIndex { id } => todo!(),
            ServiceAction::Other { custom } => unreachable!(),
        }
    }
}
