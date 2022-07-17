use std::error::Error;

use image::*;
use imggen_rs::AddPerlinNoise;
use ndarray::Array;

pub fn main() -> Result<(), Box<dyn Error>> {
    let cols = 1200;
    let rows = 1000;

    let mut img = Array::zeros([rows, cols]) + 0.5;

    img.add_perlin_noise(200, 0.7);
    img.add_perlin_noise(158, 0.7);
    img.add_perlin_noise(101, 0.3);
    img.add_perlin_noise(59, 0.2);
    img.add_perlin_noise(13, 0.05);

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
