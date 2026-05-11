#![feature(random)]

use std::{num::NonZeroU16, time::Duration};

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

pub mod client;
pub mod model;

pub enum SendMode {
    Json,
    Cbor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reliability {
    /// The channel will always retransmit
    Reliable,
    /// The channel will never retransmit
    Unreliable,
    /// The channel will retransmit this many times
    Retry(NonZeroU16),
    /// The channel will retransmit for that number of milliseconds
    ExpireAfter(NonZeroU16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChannelConfiguration {
    /// Whether the channel will be required to be ordered
    pub ordered: bool,
    /// The reliability of the channel
    pub reliability: Reliability,
}

impl Default for ChannelConfiguration {
    fn default() -> Self {
        return Self {
            ordered: true,
            reliability: Reliability::Reliable,
        };
    }
}
