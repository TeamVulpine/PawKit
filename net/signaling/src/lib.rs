#![feature(random)]

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

pub mod client;
pub mod model;

pub enum SendMode {
    Json,
    Cbor,
}
