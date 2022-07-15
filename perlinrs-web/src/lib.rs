use wasm_bindgen::prelude::*;

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

    pub fn raw_data(&self) -> Box<[u8]> {
        self.data.clone()
    }
}
