use image::save_buffer;
use perlinrs::perlin_cube;

pub fn main() {
    let mut result = vec![0; 20 * 20 * 20 * 20];
    for i in 0..10usize {
        for j in 0..10usize {
            let data = perlin_cube(i as i16, j as i16, 40, 40);
            for k in 0..40 {
                for l in 0..40 {
                    let in_value = data[k * 40 + l];
                    result[(i * 16000 + k * 400 + j * 40 + l)] = in_value;
                }
            }
        }
    }
    save_buffer("./output.png", &result, 400, 400, image::ColorType::L8)
        .expect("Error saving image");
}
