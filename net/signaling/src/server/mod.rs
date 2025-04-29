use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message, WebSocketStream};

use crate::model::SignalMessage;

mod socket;

pub async fn start_server(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    println!("Signaling server listening on {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let Ok(ws_stream) = accept_async(stream).await else {
                println!("WebSocket handshake failed");
                return;
            };
            handle_connection(ws_stream).await;
        });
    }

    Ok(())
}

async fn send_message(
    sender: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    message: SignalMessage,
) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    println!("Sending message to client: {:#?}", message);

    sender
        .send(Message::Text(
            serde_json::to_string(&message).unwrap().into(),
        ))
        .await
}

async fn handle_connection(ws_stream: WebSocketStream<TcpStream>) {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match &msg {
                Message::Text(text) => {
                    let message = serde_json::from_str::<SignalMessage>(&text).unwrap();
                    println!("Got message on server: {:#?}", message);

                    send_message(&mut ws_sender, message).await.unwrap();
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = recv_task => (),
    }
}
