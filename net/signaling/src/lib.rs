#![feature(random)]

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

pub mod client;
pub mod model;

pub enum SendMode {
    Json,
    Cbor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelConfiguration {
    /// Whether the channel will be required to be ordered
    pub ordered: bool,
    /// The number of times the packet will attempt to retransmit.
    /// If None, it always retransmits.
    pub reliability: Option<u16>,
}

impl Default for ChannelConfiguration {
    fn default() -> Self {
        return Self {
            ordered: true,
            reliability: None,
        };
    }
}
