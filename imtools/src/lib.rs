pub mod as_flat_slice;
pub mod cmaps;
pub mod image_types;
pub mod perlin;

pub mod prelude {
    pub use crate::as_flat_slice::AsFlatSlice;
    pub use crate::cmaps::*;
    pub use crate::image_types::*;
    pub use crate::perlin::*;
}

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
