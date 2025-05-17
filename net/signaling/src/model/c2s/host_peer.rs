use just_webrtc::types::{ICECandidate, SessionDescription};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HostPeerMessageC2S {
    Register {
        game_id: u32,
    },
    AcceptConnection {
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
        client_id: usize,
    },
    RejectConnection {
        client_id: usize,
    },
}
