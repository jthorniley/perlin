use js_sys::{ArrayBuffer, Float32Array, WebAssembly::Memory};
use ndarray::{Array, Dim};
use wasm_bindgen::{memory, prelude::*, JsCast};

#[wasm_bindgen]
pub struct ImageGenerator {
    data: Array<f32, Dim<[usize; 2]>>,
}

#[wasm_bindgen]
impl ImageGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize) -> ImageGenerator {
        ImageGenerator {
            data: Array::zeros((height, width)),
        }
    }

    pub fn as_array(&self) -> JsValue {
        let m: Memory = memory().unchecked_into();
        let slice = Float32Array::new_with_byte_offset_and_length(
            &m.buffer(),
            self.data.as_ptr() as u32,
            self.data.len() as u32,
        );
        slice.into()
    }
}

/*
#[wasm_bindgen]
pub struct PerlinNoise {
    data: Box<[u8]>,
}

#[wasm_bindgen]
impl PerlinNoise {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PerlinNoise {
        let mut data = Box::new([0; 100]);
        data[0] = 12;
        PerlinNoise { data }
    }

    pub fn get_data(&self) -> JsValue {
        let m: Memory = memory().unchecked_into();
        let begin = self.data.as_ptr();
        let slice = Uint8Array::new(&m.buffer());
        slice.slice(begin as u32, begin as u32 + 100).into()
    }
}
*/
