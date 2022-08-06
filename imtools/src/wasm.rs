use crate::prelude::*;

use ndarray::prelude::*;
use palette::gradient::named::VIRIDIS;

use js_sys::Uint8Array;
use wasm_bindgen::{prelude::*, Clamped};
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

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.data.fill(0.0)
    }
}

#[wasm_bindgen]
pub struct RgbaImage {
    data: Array2<[u8; 4]>,
}

#[wasm_bindgen]
impl RgbaImage {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> RgbaImage {
        let data = Array2::from_elem((height, width), [0u8, 0, 0, 0]);
        RgbaImage { data }
    }

    pub fn fill(width: usize, height: usize, r: u8, g: u8, b: u8) -> RgbaImage {
        let data = Array2::from_elem((height, width), [r, g, b, 255]);
        RgbaImage { data }
    }

    #[wasm_bindgen(js_name = "imageData")]
    pub fn image_data(&self) -> ImageData {
        ImageData::new_with_u8_clamped_array(
            Clamped(self.data.as_flat_slice()),
            self.data.dim().1 as u32,
        )
        .unwrap()
    }

    #[wasm_bindgen]
    pub fn array(&self) -> Uint8Array {
        let arr = Uint8Array::new_with_length((self.data.len() * 4) as u32);
        arr.copy_from(self.data.as_flat_slice());
        arr
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> usize {
        self.data.shape()[1]
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> usize {
        self.data.shape()[0]
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

#[wasm_bindgen]
pub struct GradientCMap;

#[wasm_bindgen]
impl GradientCMap {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GradientCMap {
        GradientCMap
    }

    #[wasm_bindgen]
    pub fn cmap(&self, image: &ScalarImage, output: &mut RgbaImage) {
        crate::cmaps::GradientCMap::new(VIRIDIS).cmap(&image.data, &mut output.data);
    }
}
