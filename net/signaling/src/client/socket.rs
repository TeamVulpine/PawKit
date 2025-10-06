use std::io::Cursor;

use pawkit_net_websocket::{Websocket, WebsocketMessage};

use crate::{
    SendMode,
    model::{c2s::SignalMessageC2S, s2c::SignalMessageS2C},
};

pub struct ClientSocket {
    sock: Websocket,
    send_mode: SendMode,
}

impl ClientSocket {
    pub async fn open(server_url: &str, send_mode: SendMode) -> Option<Self> {
        let sock = Websocket::new(server_url).await?;

        return Some(Self { sock, send_mode });
    }

    pub async fn recv(&mut self) -> Option<SignalMessageS2C> {
        let msg = self.sock.recv().await?;

        return match msg {
            WebsocketMessage::String(msg) => serde_json::from_str(&msg).unwrap(),
            WebsocketMessage::Array(msg) => ciborium::from_reader(Cursor::new(msg)).unwrap(),
        };
    }

    pub fn is_open(&self) -> bool {
        return self.sock.is_open();
    }

    pub async fn send(&mut self, message: SignalMessageC2S) -> Option<()> {
        return self
            .sock
            .send(match self.send_mode {
                SendMode::Json => {
                    WebsocketMessage::String(serde_json::to_string(&message).unwrap())
                }

                SendMode::Cbor => {
                    let mut data: Vec<u8> = vec![];

                    ciborium::into_writer(&message, &mut data).unwrap();

                    WebsocketMessage::Array(data)
                }
            })
            .await
            .ok();
    }
}
