#![feature(let_chains)]

mod client;
mod host;

use std::{future::Future, sync::LazyLock};

pub use client::*;
pub use host::*;
use just_webrtc::{platform::Channel, DataChannelExt};
use tokio::runtime::Runtime;

#[cfg(not(target_arch = "wasm32"))]
type PacketFuture = dyn Future<Output = (Option<Vec<u8>>, usize)> + Send + Sync;
#[cfg(target_arch = "wasm32")]
type PacketFuture = dyn Future<Output = (Option<Vec<u8>>, usize)>;

async fn recieve_packet(channel: &Channel) -> Option<Vec<u8>> {
    return channel.receive().await.map(|it| it.to_vec()).ok();
}

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());
