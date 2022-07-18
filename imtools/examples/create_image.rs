use std::error::Error;

use image::*;
use imtools::{
    cmaps::{CMap, Grayscale},
    perlin::Perlin,
};
use ndarray::{Array, Dim};

pub fn main() -> Result<(), Box<dyn Error>> {
    let cols = 1200;
    let rows = 1000;

    let mut img: Array<f32, Dim<[usize; 2]>> = Array::zeros([rows, cols]);

    Perlin::new(200, 0.7).add_to_image(&mut img);
    Perlin::new(158, 0.7).add_to_image(&mut img);
    Perlin::new(101, 0.1).add_to_image(&mut img);
    Perlin::new(59, 0.1).add_to_image(&mut img);
    Perlin::new(13, 0.1).add_to_image(&mut img);
    Perlin::new(5, 0.1).add_to_image(&mut img);

    let result = Grayscale.cmap(&img);

    save_buffer(
        "./output.png",
        result.as_slice().ok_or("Unexpected error")?,
        cols as u32,
        rows as u32,
        image::ColorType::Rgba8,
    )?;

    Ok(())
}
