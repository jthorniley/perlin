use std::error::Error;

use image::*;
use ndarray::Array;
use perlinrs::AddPerlinNoise;

pub fn main() -> Result<(), Box<dyn Error>> {
    let cols = 1200;
    let rows = 1000;

    let mut img = Array::zeros([rows, cols]) + 0.7;

    img.add_perlin_noise(20, 0.4);
    img.add_perlin_noise(3, 0.4);
    img.add_perlin_noise(6, 0.2);

    let result = img.map(|value: &f32| (value.clamp(0.0, 1.0) * 255.0) as u8);

    save_buffer(
        "./output.png",
        result.as_slice().ok_or("Unexpected error")?,
        cols as u32,
        rows as u32,
        image::ColorType::L8,
    )?;

    Ok(())
}
