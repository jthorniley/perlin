use std::error::Error;

use image::*;
use imtools::{
    cmaps::{CMap, GradientCMap},
    perlin::Perlin,
};
use ndarray::{Array, Dim};
use palette::gradient::named::MAGMA;

pub fn main() -> Result<(), Box<dyn Error>> {
    let cols = 1200;
    let rows = 1000;

    let mut img: Array<f32, Dim<[usize; 2]>> = Array::zeros([rows, cols]);

    Perlin::new(400, 0.9).add_to_image(&mut img);
    Perlin::new(158, 0.3).add_to_image(&mut img);
    Perlin::new(101, 0.3).add_to_image(&mut img);
    Perlin::new(59, 0.1).add_to_image(&mut img);
    Perlin::new(3, 0.1).add_to_image(&mut img);

    let result = GradientCMap::new(MAGMA).cmap(&img);

    let mut image_buffer: RgbImage = ImageBuffer::new(cols as u32, rows as u32);

    image_buffer
        .enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            *pixel = Rgb::from(*result.get((y as usize, x as usize)).unwrap());
        });

    image_buffer.save("output.png")?;

    Ok(())
}
