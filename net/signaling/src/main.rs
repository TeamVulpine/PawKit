#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    use pawkit_net_signaling::server::SimpleSignalingServer;

    pawkit_logger::info("Starting signaling server");

    SimpleSignalingServer::new("localhost:1234", "ws://localhost:1234".into())
        .await
        .unwrap()
        .start()
        .await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    pawkit_logger::error("Signaling server cannot be run in wasm32 target.");
}
