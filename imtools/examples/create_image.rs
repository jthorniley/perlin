use std::error::Error;

use image::*;
use imtools::prelude::*;
use ndarray::prelude::*;
use palette::convert::TryIntoColor;

use palette::gradient::Gradient;
use palette::{Hsl, Srgb};

pub fn main() -> Result<(), Box<dyn Error>> {
    let colors: Vec<Srgb> = vec![
        Hsl::new(85.0, 0.2, 0.5).try_into_color().unwrap(),
        Hsl::new(75.0, 0.3, 0.5).try_into_color().unwrap(),
        Hsl::new(65.0, 0.3, 0.7).try_into_color().unwrap(),
    ];
    let grad = Gradient::new(colors.into_iter().map(|col| col.into_linear()));
    let cols = 1200;
    let rows = 1000;

    let mut img: Array2<f32> = Array::zeros([rows, cols]);

    Perlin::new(400, 0.6).add_to_image(&mut img);
    Perlin::new(158, 0.5).add_to_image(&mut img);
    Perlin::new(101, 0.3).add_to_image(&mut img);
    Perlin::new(59, 0.2).add_to_image(&mut img);

    let result = GradientCMap::new(grad).cmap(&img);

    let image_buffer =
        RgbaImage::from_raw(cols as u32, rows as u32, result.as_flat_slice().to_vec()).unwrap();

    image_buffer.save("output.png")?;

    Ok(())
}
