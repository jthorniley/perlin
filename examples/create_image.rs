use std::error::Error;

use image::*;
use ndarray::Array;
use perlinrs::AddPerlinNoise;

pub fn main() -> Result<(), Box<dyn Error>> {
    let cols = 400;
    let rows = 200;

    let mut img = Array::zeros([rows, cols]);

    img.add_perlin_noise(15);

    let result = img.map(|value: &f32| ((value / 2.0 + 0.5).clamp(0.0, 1.0) * 255.0) as u8);

    save_buffer(
        "./output.png",
        result.as_slice().ok_or("Unexpected error")?,
        cols as u32,
        rows as u32,
        image::ColorType::L8,
    )?;

    Ok(())
}
