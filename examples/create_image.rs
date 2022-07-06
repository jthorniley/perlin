#[macro_use]
extern crate ndarray;

use std::{error::Error, ops::Add};

use image::*;
use ndarray::prelude::*;
use perlinrs::{hash, new_square};

/*
pub fn main() {
    let w = 400u32;
    let h = 400u32;

    let a = noise_2d(w as usize, h as usize, 80);
    let b = noise_2d(w as usize, h as usize, 14);
    let imgbuf = a
        .iter()
        .zip(b.iter())
        .map(|(a, b)| a * 0.8 + b * 0.1)
        .map(|x| ((x + 1.0) * 128.0) as u8)
        .collect::<Vec<u8>>();
    save_buffer("./output.png", &imgbuf, w, h, image::ColorType::L8).expect("Error saving image");
}
*/

pub fn main() -> Result<(), Box<dyn Error>> {
    let w = 400;
    let h = 200;

    let hashes = Array::range(0., 100., 1.).map(|x| hash(*x as usize));
    println!("Hash values: {:?}", hashes);

    let mut img = Array::zeros([h, w]);

    new_square(&mut img.slice_mut(s![20..30, 5..200]));
    let result = img.map(|value: &f32| (value.clamp(0.0, 1.0) * 255.0) as u8);

    save_buffer(
        "./output.png",
        result.as_slice().ok_or("Unexpected error")?,
        w as u32,
        h as u32,
        image::ColorType::L8,
    )?;

    Ok(())
}
