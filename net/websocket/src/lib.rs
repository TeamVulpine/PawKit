#![feature(decl_macro)]

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[derive(Debug)]
pub enum WebsocketMessage {
    String(String),
    Array(Vec<u8>),
}

pub enum WebsocketError {
    InvalidState,
    NotOpen,
}
