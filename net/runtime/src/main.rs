use pawkit_net_runtime::NetHostPeer;

fn main() {
    let (peer, mut recv) = NetHostPeer::create("ws://localhost:1234", 32).unwrap();

    while let Some(msg) = recv.blocking_recv() {
        pawkit_logger::info(&format!("{:#?}", msg));
    }

    peer.shutdown();
}
