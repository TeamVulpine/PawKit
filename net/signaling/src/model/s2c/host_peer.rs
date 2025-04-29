use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HostPeerMessageS2C {
    Registered {
        /// TODO: make a GameId struct
        game_id: String,
    },
}
