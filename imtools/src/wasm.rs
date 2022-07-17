use js_sys::{Float32Array, WebAssembly::Memory};
use ndarray::{Array, Dim};
use wasm_bindgen::{memory, prelude::*, JsCast};

#[wasm_bindgen]
pub struct ScalarImage {
    data: Array<f32, Dim<[usize; 2]>>,
}

#[wasm_bindgen]
impl ScalarImage {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> ScalarImage {
        let data = Array::zeros((height, width));
        ScalarImage { data }
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
}


#[wasm_bindgen]
pub struct Perlin {
    inner: crate::perlin::Perlin<f32>
}

#[wasm_bindgen]
impl Perlin {

    #[wasm_bindgen(constructor)]
    pub fn new(scale: usize, amp: f32) -> Perlin {
        Perlin { inner: crate::perlin::Perlin::<f32>::new(scale, amp) }
    }


    #[wasm_bindgen(js_name="addToImage")]
    pub fn add_to_image(&self, image: &mut ScalarImage) {
        self.inner.add_to_image(&mut image.data)
    }
}