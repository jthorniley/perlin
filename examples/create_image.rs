use image::save_buffer;
use perlinrs::noise_2d;

pub fn main() {
    let result = noise_2d(400, 400, 40);
    let imgbuf = result
        .iter()
        .map(|x| ((x + 1.0) * 128.0) as u8)
        .collect::<Vec<u8>>();
    save_buffer("./output.png", &imgbuf, 400, 400, image::ColorType::L8)
        .expect("Error saving image");
}
