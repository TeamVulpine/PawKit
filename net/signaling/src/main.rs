#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use pawkit_net_websocket::{Websocket, WebsocketMessage};

    let mut sock = Websocket::new("wss://echo.websocket.org").await.unwrap();

    let _ = sock
        .send(WebsocketMessage::String("Hello, world!".into()))
        .await;
    let _ = sock
        .send(WebsocketMessage::Array(vec![1, 2, 3, 4, 5, 6]))
        .await;

    while let Some(message) = sock.recv().await {
        println!("{:?}", message);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("Signaling server doesn't support WASM.");
}
