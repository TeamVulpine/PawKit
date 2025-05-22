use just_webrtc::types::{ICECandidate, SessionDescription};
use serde::{Deserialize, Serialize};

use crate::model::HostId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HostPeerMessageS2C {
    Registered {
        host_id: HostId,
    },
    ConnectionRequested {
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
        client_id: u64,
    },
}
