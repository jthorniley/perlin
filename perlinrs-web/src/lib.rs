use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_fn() -> u32 {
    println!("Test");
    23
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
