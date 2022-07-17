use js_sys::{Float32Array, WebAssembly::Memory};
use ndarray::{Array, Dim};
use perlinrs::AddPerlinNoise;
use wasm_bindgen::{memory, prelude::*, JsCast};

#[wasm_bindgen]
pub struct ImageGenerator {
    data: Array<f32, Dim<[usize; 2]>>,
}

#[wasm_bindgen]
impl ImageGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> ImageGenerator {
        let data = Array::zeros((height, width));
        ImageGenerator { data }
    }

    /// Get the underlying array of data for the image.
    ///
    /// Returns a float array of the randomly generated image.
    #[wasm_bindgen(js_name = "imageData")]
    pub fn image_data(&self) -> Float32Array {
        Float32Array::new_with_byte_offset_and_length(
            &memory().unchecked_into::<Memory>().buffer(),
            self.data.as_ptr() as u32,
            self.data.len() as u32,
        )
    }

    // Add randomly generated perlin noise to image
    #[wasm_bindgen(js_name = "addPerlinNoise")]
    pub fn add_perlin_noise(&mut self, scale: usize, amp: f32) {
        self.data.add_perlin_noise(scale, amp);
    }
}
