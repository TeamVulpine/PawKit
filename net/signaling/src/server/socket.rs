use std::io::Cursor;

use futures_util::{stream::FusedStream, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::{
    model::{c2s::SignalMessageC2S, s2c::SignalMessageS2C},
    SendMode,
};

pub struct ServerSocket {
    sock: WebSocketStream<TcpStream>,
    send_mode: SendMode,
}

impl ServerSocket {
    pub fn new(sock: WebSocketStream<TcpStream>, send_mode: SendMode) -> Self {
        return Self { sock, send_mode };
    }

    pub fn running(&self) -> bool {
        return !self.sock.is_terminated();
    }

    pub async fn recv(&mut self) -> Option<SignalMessageC2S> {
        let msg = self.sock.next().await?.ok()?;

        return match msg {
            Message::Text(msg) => serde_json::from_str(&msg).unwrap(),
            Message::Binary(msg) => ciborium::from_reader(Cursor::new(msg.to_vec())).unwrap(),
            _ => todo!(),
        };
    }

    pub async fn send(&mut self, message: SignalMessageS2C) -> Option<()> {
        return self
            .sock
            .send(match self.send_mode {
                SendMode::Json => Message::Text(serde_json::to_string(&message).unwrap().into()),
                SendMode::Cbor => {
                    let mut data: Vec<u8> = vec![];

                    ciborium::into_writer(&message, &mut data).unwrap();

                    Message::Binary(data.into())
                }
            })
            .await
            .ok();
    }
}
