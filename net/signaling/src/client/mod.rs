use std::io::Cursor;

use pawkit_net_websocket::{Websocket, WebsocketMessage};

use crate::{
    model::{c2s::SignalMessageC2S, s2c::SignalMessageS2C},
    SendMode,
};

pub struct ClientSocket {
    sock: Websocket,
    send_mode: SendMode,
}

impl ClientSocket {
    pub async fn recv(&mut self) -> Option<SignalMessageS2C> {
        let msg = self.sock.recv().await?;

        return match msg {
            WebsocketMessage::String(msg) => serde_json::from_str(&msg).unwrap(),
            WebsocketMessage::Array(msg) => ciborium::from_reader(Cursor::new(msg)).unwrap(),
        };
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
