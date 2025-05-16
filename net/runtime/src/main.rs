use anyhow::Result;
use just_webrtc::{
    types::{ICECandidate, PeerConnectionState, SessionDescription},
    DataChannelExt, PeerConnectionExt, SimpleLocalPeerConnection, SimpleRemotePeerConnection,
};

#[tokio::main]
async fn main() {}

async fn run_local_peer() -> Result<()> {
    // create simple local peer connection with unordered data channel
    let local_peer_connection = SimpleLocalPeerConnection::build(false).await?;

    // output offer and candidates for remote peer
    let offer = local_peer_connection.get_local_description().await.unwrap();
    let candidates = local_peer_connection.collect_ice_candidates().await?;

    // ... send the offer and the candidates to Peer B via external signalling implementation ...
    let signalling = (offer, candidates);

    // ... receive the answer and candidates from Peer B via external signalling implementation ...
    let (answer, candidates) = signalling;

    // update local peer from received answer and candidates
    local_peer_connection.set_remote_description(answer).await?;
    local_peer_connection.add_ice_candidates(candidates).await?;

    // local signalling is complete! we can now wait for a complete connection
    while local_peer_connection.state_change().await != PeerConnectionState::Connected {}

    // receive data channel from local peer
    let local_channel = local_peer_connection.receive_channel().await.unwrap();
    // wait for data channels to be ready
    local_channel.wait_ready().await;

    // send data to remote (answerer)
    local_channel
        .send(&bytes::Bytes::from("hello remote!"))
        .await?;
    // recv data from remote (answerer)
    let recv = local_channel.receive().await.unwrap();
    assert_eq!(&recv, "hello local!");

    Ok(())
}

async fn run_remote_peer(offer: SessionDescription, candidates: Vec<ICECandidate>) -> Result<()> {
    // ... receive the offer and the candidates from Peer A via external signalling implementation ...

    // create simple remote peer connection from received offer and candidates
    let remote_peer_connection = SimpleRemotePeerConnection::build(offer).await?;
    remote_peer_connection
        .add_ice_candidates(candidates)
        .await?;
    // output answer and candidates for local peer
    let answer = remote_peer_connection
        .get_local_description()
        .await
        .unwrap();
    let candidates = remote_peer_connection.collect_ice_candidates().await?;

    // ... send the answer and the candidates back to Peer A via external signalling implementation ...
    let _signalling = (answer, candidates);

    // remote signalling is complete! we can now wait for a complete connection
    while remote_peer_connection.state_change().await != PeerConnectionState::Connected {}

    // receive data channel from local and remote peers
    let remote_channel = remote_peer_connection.receive_channel().await.unwrap();
    // wait for data channels to be ready
    remote_channel.wait_ready().await;

    // send/recv data from local (offerer) to remote (answerer)
    let recv = remote_channel.receive().await.unwrap();
    assert_eq!(&recv, "hello remote!");
    // send/recv data from remote (answerer) to local (offerer)
    remote_channel
        .send(&bytes::Bytes::from("hello local!"))
        .await?;

    Ok(())
}
