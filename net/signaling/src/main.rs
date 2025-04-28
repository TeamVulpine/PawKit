#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use std::{thread::sleep, time::Duration};

    use pawkit_net_signaling::model::SignalMessage;
    use pawkit_net_websocket::{Websocket, WebsocketMessage};

    let _server = tokio::spawn(pawkit_net_signaling::server::start_server("localhost:9001"));

    // Delay to allow server to start up
    sleep(Duration::from_secs(3));

    println!("Connecting with client");

    let mut sock = Websocket::new("ws://localhost:9001").await.unwrap();

    let message = SignalMessage::Test {
        message: "wawa".into(),
    };

    println!("Sending message to server: {:#?}", message);

    let _ = sock
        .send(WebsocketMessage::String(
            serde_json::to_string(&message).unwrap(),
        ))
        .await;

    while let Some(message) = sock.recv().await {
        let WebsocketMessage::String(message) = message else {
            continue;
        };

        let message = serde_json::from_str::<SignalMessage>(&message).unwrap();

        println!("Got message on client: {:#?}", message);

        break;
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("Signaling server doesn't support WASM.");
}
