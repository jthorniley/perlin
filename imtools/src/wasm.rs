use crate::prelude::*;

use ndarray::prelude::*;
use palette::gradient::named::VIRIDIS;

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
        ImageData::new_with_u8_clamped_array(
            Clamped(self.data.as_flat_slice()),
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

#[wasm_bindgen]
pub struct GradientCMap;

#[wasm_bindgen]
impl GradientCMap {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GradientCMap {
        GradientCMap
    }

    #[wasm_bindgen]
    pub fn cmap(&self, image: &ScalarImage) -> RgbaImage {
        let data = crate::cmaps::GradientCMap::new(VIRIDIS).cmap(&image.data);
        RgbaImage { data }
    }
}
