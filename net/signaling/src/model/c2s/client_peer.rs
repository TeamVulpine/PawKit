use just_webrtc::types::{ICECandidate, SessionDescription};
use serde::{Deserialize, Serialize};

use crate::model::HostId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPeerMessageC2S {
    Connect {
        game_id: u32,
        host_id: HostId,
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
    },
}
