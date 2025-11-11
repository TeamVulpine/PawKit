use just_webrtc::types::{ICECandidate, SessionDescription};
use serde::{Deserialize, Serialize};

use crate::model::HostId;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPeerMessageC2S {
    RequestConnection {
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
        host_id: HostId,
        game_id: u32,
    },
    RequestChannelConfigurations {
        host_id: HostId,
        game_id: u32,
    },
}
