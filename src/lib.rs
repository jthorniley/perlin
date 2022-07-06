use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use ndarray::{ArrayBase, DataMut, Dim};

pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    fn from_direction(direction: f32) -> Vec2 {
        let (x, y) = direction.sin_cos();
        Vec2 { x, y }
    }

    fn negate_add_dot(&self, a: f32, b: f32, other: &Vec2) -> f32 {
        return (a - self.x) * other.x + (b - self.y) * other.y;
    }

    fn translate_x(&mut self, dx: f32) {
        self.x += dx;
    }
    fn translate_y(&mut self, dy: f32) {
        self.y += dy;
    }
}

fn corner_vector(x: usize, y: usize) -> Vec2 {
    let mut hasher = DefaultHasher::new();
    hasher.write_usize(x);
    hasher.write_usize(y);
    let hash_value = hasher.finish() as u8;
    let direction = hash_value as f32 / u8::MAX as f32;
    Vec2::from_direction(direction * std::f32::consts::PI * 2.0)
}

fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}

fn square(x: usize, y: usize, scale: usize, buf: &mut Vec<f32>, offset: usize, stride: usize) {
    let corners = [
        corner_vector(x, y),
        corner_vector(x, y + 1),
        corner_vector(x + 1, y),
        corner_vector(x + 1, y + 1),
    ];
    let step = 1.0 / scale as f32;
    let mut pixel = Vec2::new(step / 2.0, step / 2.0);

    for i in 0..scale {
        for j in 0..scale {
            let idx = j + stride * i + offset;

            if j != 0 && idx % stride == 0 {
                // If idx % stride = 0, then this pixel is at the start
                // of a row, which should only happen if j == 0 - if j isn't
                // 0, this means we have wrapped around the side of the image,
                // and can stop adding to the buffer for the current row
                break;
            }

            let p1 = pixel.negate_add_dot(0.0, 0.0, &corners[0]);
            let p2 = pixel.negate_add_dot(0.0, 1.0, &corners[1]);
            let p3 = pixel.negate_add_dot(1.0, 0.0, &corners[2]);
            let p4 = pixel.negate_add_dot(1.0, 1.0, &corners[3]);

            let interp1 = interpolate(p1, p2, pixel.y);
            let interp2 = interpolate(p3, p4, pixel.y);
            let interp = interpolate(interp1, interp2, pixel.x);

            if idx < buf.len() {
                buf[idx] = interp;
            }

            pixel.translate_x(step);
        }
        pixel.translate_y(step);
        pixel.x = step / 2.0;
    }
}

pub fn noise_2d(width: usize, height: usize, scale: usize) -> Vec<f32> {
    let mut buf = vec![0.0; width * height];

    let rx = (width - 1) / scale + 1;
    let ry = (height - 1) / scale + 1;

    let xstride = scale;
    let ystride = width * scale;

    for y in 0..ry {
        for x in 0..rx {
            square(x, y, scale, &mut buf, x * xstride + y * ystride, width)
        }
    }

    buf
}

pub fn hash(value: usize) -> u8 {
    let key: u32 = 0x730d319b;
    let key1: u32 = 0x6373cd28;

    let mut keyed = value as u32;
    keyed ^= keyed.rotate_left(3) ^ keyed.rotate_left(17) ^ key;
    keyed ^= keyed.rotate_left(5) ^ keyed.rotate_left(27) ^ key1;

    keyed.rotate_right((value & 0xf) as u32) as u8
}

pub fn new_square<E: From<f32> + Copy, S: DataMut<Elem = E>>(
    buffer: &mut ArrayBase<S, Dim<[usize; 2]>>,
) {
    buffer.map_inplace(|x| *x = 100.0f32.into());
}

#[cfg(test)]
mod tests {}
