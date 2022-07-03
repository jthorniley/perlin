use image::save_buffer;
use perlinrs::perlin_cube;

pub fn main() {
    let mut result = vec![0; 10 * 40 * 10 * 40];
    for i in 0..10usize {
        for j in 0..10usize {
            perlin_cube(
                i as i16,
                j as i16,
                40,
                40,
                &mut result,
                i * 40 + j * 16000,
                400,
            );
        }
    }
    save_buffer("./output.png", &result, 400, 400, image::ColorType::L8)
        .expect("Error saving image");
}
