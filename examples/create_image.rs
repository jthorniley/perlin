use image::save_buffer;
use perlinrs::perlin_cube;

pub fn main() {
    let data = perlin_cube(0, 0, 100, 140);
    save_buffer("./output.png", &data, 100, 140, image::ColorType::L8).expect("Error saving image");
}
