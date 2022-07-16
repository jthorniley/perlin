use js_sys::{Uint8Array, WebAssembly::Memory};
use wasm_bindgen::{memory, prelude::*, JsCast};

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
