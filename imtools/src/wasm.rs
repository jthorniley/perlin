use js_sys::{Float32Array, Uint8Array, Uint8ClampedArray, WebAssembly::Memory};
use ndarray::prelude::*;
use wasm_bindgen::{memory, prelude::*, Clamped, JsCast};
use web_sys::ImageData;

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
pub struct RgbaImage {
    data: Array2<[u8; 4]>,
}

#[wasm_bindgen]
impl RgbaImage {
    pub fn fill(width: usize, height: usize, r: u8, g: u8, b: u8) -> RgbaImage {
        let data = Array2::from_elem((height, width), [r, g, b, 255]);
        RgbaImage { data }
    }

    #[wasm_bindgen(js_name = "imageData")]
    pub fn image_data(&self) -> ImageData {
        let mut data = Array1::zeros(self.data.len() * 4);
        self.data
            .as_slice()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, val)| {
                data.slice_mut(s![i * 4..(i + 1) * 4])
                    .zip_mut_with(&Array1::from_iter(val), |a, b| *a = **b);
            });
        ImageData::new_with_u8_clamped_array(
            Clamped(data.as_slice().unwrap()),
            self.data.dim().1 as u32,
        )
        .unwrap()
    }
}

#[wasm_bindgen]
pub struct Perlin {
    inner: crate::perlin::Perlin<f32>,
}

#[wasm_bindgen]
impl Perlin {
    #[wasm_bindgen(constructor)]
    pub fn new(scale: usize, amp: f32) -> Perlin {
        Perlin {
            inner: crate::perlin::Perlin::<f32>::new(scale, amp),
        }
    }

    #[wasm_bindgen(js_name = "addToImage")]
    pub fn add_to_image(&self, image: &mut ScalarImage) {
        self.inner.add_to_image(&mut image.data)
    }
}
