use just_webrtc::types::{ICECandidate, SessionDescription};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPeerMessageS2C {
    ConnectionAccepted {
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
    },
    ConnectionRejected,
}
