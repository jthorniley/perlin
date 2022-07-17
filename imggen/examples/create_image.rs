use std::error::Error;

use image::*;
use imggen_rs::{AddPerlinNoise, Grayscale, MapToRgba};
use ndarray::{Array, Dim};

pub fn main() -> Result<(), Box<dyn Error>> {
    let cols = 1200;
    let rows = 1000;

    let mut img: Array<f32, Dim<[usize; 2]>> = Array::zeros([rows, cols]);

    img.add_perlin_noise(400, 0.7);
    img.add_perlin_noise(158, 0.7);
    img.add_perlin_noise(101, 0.1);
    img.add_perlin_noise(59, 0.1);
    img.add_perlin_noise(13, 0.1);
    img.add_perlin_noise(5, 0.1);

    let result = img.map_to_rgba(&Grayscale);

    save_buffer(
        "./output.png",
        result.as_slice().ok_or("Unexpected error")?,
        cols as u32,
        rows as u32,
        image::ColorType::Rgba8,
    )?;

    Ok(())
}
