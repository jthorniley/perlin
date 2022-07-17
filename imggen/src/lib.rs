pub mod cmaps;
mod imggen;

pub use cmaps::{Grayscale, MapToRgba};
pub use imggen::*;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
