use futures_util::{stream::FusedStream, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::{WebsocketError, WebsocketMessage};

pub struct Websocket {
    raw_sock: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Websocket {
    pub async fn new(url: &str) -> Option<Self> {
        let (sock, _) = match connect_async(url).await {
            Ok(value) => value,
            Err(err) => {
                pawkit_logger::error(&format!("{}", err));
                return None;
            }
        };

        return Some(Self { raw_sock: sock });
    }

    pub fn is_open(&self) -> bool {
        return !self.raw_sock.is_terminated();
    }

    pub async fn close(&mut self) {
        self.raw_sock.close(None).await.unwrap();
    }

    pub async fn recv(&mut self) -> Option<WebsocketMessage> {
        if !self.is_open() {
            return None;
        }

        loop {
            let Some(Ok(message)) = self.raw_sock.next().await else {
                return None;
            };

            match message {
                Message::Text(text) => {
                    return Some(WebsocketMessage::String(text.as_str().into()));
                }

                Message::Binary(bin) => {
                    return Some(WebsocketMessage::Array(bin.to_vec()));
                }

                _ => {}
            }
        }
    }

    pub async fn send(&mut self, message: WebsocketMessage) -> Result<(), WebsocketError> {
        if !self.is_open() {
            return Err(WebsocketError::NotOpen);
        }

        match message {
            WebsocketMessage::String(str) => {
                if self.raw_sock.send(Message::Text(str.into())).await.is_err() {
                    return Err(WebsocketError::InvalidState);
                }

                return Ok(());
            }

            WebsocketMessage::Array(arr) => {
                if self
                    .raw_sock
                    .send(Message::Binary(arr.into()))
                    .await
                    .is_err()
                {
                    return Err(WebsocketError::InvalidState);
                }

                return Ok(());
            }
        }
    }
}
