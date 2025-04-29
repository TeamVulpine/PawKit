use client_peer::ClientPeerMessageC2S;
use host_peer::HostPeerMessageC2S;
use serde::{Deserialize, Serialize};

use super::SignalingError;

pub mod client_peer;
pub mod host_peer;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SignalMessageC2S {
    HostPeer { value: HostPeerMessageC2S },
    ClientPeer { value: ClientPeerMessageC2S },
    Error { value: SignalingError },
}
