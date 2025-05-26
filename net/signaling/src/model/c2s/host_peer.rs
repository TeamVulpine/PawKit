use just_webrtc::types::{ICECandidate, SessionDescription};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HostPeerMessageC2S {
    Register {
        game_id: u32,
        /// Whether to request a proxy.
        ///
        /// Being proxied means that the server holds its own WebRTC peers as well,
        /// avoiding the IP of the host address being leaked.
        ///
        /// Requesting a proxy does not guarantee you get one.
        request_proxy: bool,
    },
    AcceptConnection {
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
        client_id: u64,
    },
    RejectConnection {
        client_id: u64,
    },
}
