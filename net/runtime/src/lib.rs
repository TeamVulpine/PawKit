mod client;
mod host;

use std::{future::Future, sync::LazyLock};

pub use client::*;
use futures_util::{FutureExt, future::select_all};
pub use host::*;
use just_webrtc::{
    DataChannelExt, PeerConnectionExt,
    platform::{Channel, PeerConnection},
};
use tokio::runtime::Runtime;

#[cfg(not(target_arch = "wasm32"))]
type PacketFuture = dyn Future<Output = (Option<(usize, Vec<u8>)>, usize)> + Send + Sync;
#[cfg(target_arch = "wasm32")]
type PacketFuture = dyn Future<Output = (Option<(usize, Vec<u8>)>, usize)>;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

struct Connection {
    pub raw_connection: PeerConnection,
    pub channels: Box<[Channel]>,
}

impl Connection {
    pub async fn from(
        raw_connection: PeerConnection,
        channels: usize,
    ) -> Result<Self, just_webrtc::platform::Error> {
        let mut c = vec![];

        for _ in 0..channels {
            c.push(raw_connection.receive_channel().await?);
        }

        return Ok(Self {
            channels: c.into_boxed_slice(),
            raw_connection,
        });
    }
}

async fn receive_packet(channel: &Channel) -> Option<Vec<u8>> {
    return channel.receive().await.map(|it| it.to_vec()).ok();
}

async fn receive_packets(channels: &[Channel]) -> Option<(usize, Vec<u8>)> {
    let futures = channels.iter().map(|ch| receive_packet(ch).boxed());

    let (result, idx, _remaining) = select_all(futures).await;

    result.map(|data| (idx, data))
}
