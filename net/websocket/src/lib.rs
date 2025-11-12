#![feature(decl_macro)]

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod native;

use thiserror::Error;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[derive(Debug)]
pub enum WebsocketMessage {
    String(String),
    Array(Vec<u8>),
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WebsocketError {
    #[error("Invalid websocket state")]
    InvalidState,
    #[error("Websocket not open")]
    NotOpen,
}
