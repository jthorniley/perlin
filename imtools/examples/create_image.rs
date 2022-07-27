use std::{error::Error, slice};

use image::*;
use imtools::{
    cmaps::{CMap, GradientCMap},
    perlin::Perlin,
};
use ndarray::prelude::*;
use palette::gradient::named::MAGMA;

fn flatten(input: &Array2<[u8; 4]>) -> &[u8] {
    unsafe { slice::from_raw_parts(input.as_ptr() as *const u8, input.len() * 4) }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let cols = 1200;
    let rows = 1000;

    let mut img: Array2<f32> = Array::zeros([rows, cols]);

    Perlin::new(400, 0.9).add_to_image(&mut img);
    Perlin::new(158, 0.3).add_to_image(&mut img);
    Perlin::new(101, 0.3).add_to_image(&mut img);
    Perlin::new(59, 0.1).add_to_image(&mut img);
    Perlin::new(3, 0.1).add_to_image(&mut img);

    let result = GradientCMap::new(MAGMA).cmap(&img);

    let image_buffer =
        RgbaImage::from_raw(cols as u32, rows as u32, flatten(&result).to_vec()).unwrap();

    image_buffer.save("output.png")?;

    Ok(())
}
