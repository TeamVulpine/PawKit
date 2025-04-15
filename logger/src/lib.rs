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
#[repr(u8)]
pub enum LogLevel {
    Info,
    Error,
    Debug,
    Warn,
    Fatal,
}
