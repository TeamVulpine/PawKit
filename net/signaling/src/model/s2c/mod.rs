use client_peer::ClientPeerMessageS2C;
use host_peer::HostPeerMessageS2C;
use serde::{Deserialize, Serialize};

use super::SignalingError;

pub mod client_peer;
pub mod host_peer;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SignalMessageS2C {
    HostPeer { value: HostPeerMessageS2C },
    ClientPeer { value: ClientPeerMessageS2C },
    Error { value: SignalingError },
}
