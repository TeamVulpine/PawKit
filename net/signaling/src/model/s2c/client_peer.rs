use just_webrtc::types::{ICECandidate, SessionDescription};
use serde::{Deserialize, Serialize};

use crate::model::ChannelConfiguration;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPeerMessageS2C {
    ConnectionAccepted {
        offer: SessionDescription,
        candidates: Vec<ICECandidate>,
    },
    ChannelConfigurations(Vec<ChannelConfiguration>),
    ConnectionRejected,
}
