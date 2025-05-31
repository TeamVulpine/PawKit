use pawkit_net_signaling::server::SimpleSignalingServer;
use std::env;

const URL_ENV: &str = "PAWKIT_SIGNALING_URL";
const PORT_ENV: &str = "PAWKIT_SIGNALING_PORT";

const INSECURE_LOCALHOST_URL: &str = "ws://localhost:8080";
const SECURE_LOCALHOST_URL: &str = "wss://localhost:8080";

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    pawkit_logger::info("Starting signaling server");

    let port_string = env::var(PORT_ENV).unwrap_or("8080".into());
    let url_string = env::var(URL_ENV).unwrap_or(if SimpleSignalingServer::has_tls_evn() {
        INSECURE_LOCALHOST_URL.into()
    } else {
        SECURE_LOCALHOST_URL.into()
    });

    SimpleSignalingServer::new(&format!("localhost:{}", port_string), url_string.into())
        .await
        .unwrap()
        .start()
        .await;
}

#[cfg(target_arch = "wasm32")]
fn main() {
    pawkit_logger::error("Signaling server cannot be run in wasm32 target.");
}
