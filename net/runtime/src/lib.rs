mod client;
mod host;

use std::future::Future;

pub use client::*;
pub use host::*;
use just_webrtc::{
    DataChannelExt, PeerConnectionExt,
    platform::{Channel, PeerConnection},
};

#[cfg(not(target_arch = "wasm32"))]
type PacketFuture = dyn Future<Output = (Option<Vec<u8>>, usize)> + Send + Sync;
#[cfg(target_arch = "wasm32")]
type PacketFuture = dyn Future<Output = (Option<Vec<u8>>, usize)>;

struct Connection {
    pub raw_connection: PeerConnection,
    pub channel: Channel,
}

impl Connection {
    pub async fn from(
        raw_connection: PeerConnection,
    ) -> Result<Self, just_webrtc::platform::Error> {
        return Ok(Self {
            channel: raw_connection.receive_channel().await?,
            raw_connection,
        });
    }
}

async fn recieve_packet(channel: &Channel) -> Option<Vec<u8>> {
    return channel.receive().await.map(|it| it.to_vec()).ok();
}
