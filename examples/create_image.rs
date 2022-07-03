use image::save_buffer;
use perlinrs::noise_2d;

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
