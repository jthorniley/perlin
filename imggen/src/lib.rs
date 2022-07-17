mod imggen;

#[cfg(not(target_arch = "wasm32"))]
pub use imggen::*;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;
