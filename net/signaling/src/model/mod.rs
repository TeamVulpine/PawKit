use serde::{Deserialize, Serialize};

pub mod c2s;
pub mod s2c;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SignalMessage {
    Test { message: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignalingError {
    InvalidExpectedMessage,
}

pub struct PeerId {
    pub server_url: String,
    pub shard_id: u8,
    pub game_id: u32,
}
