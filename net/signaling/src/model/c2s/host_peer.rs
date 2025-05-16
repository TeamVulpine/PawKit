use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HostPeerMessageC2S {
    Register { game_id: u32 },
}
