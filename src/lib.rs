use std::num::Wrapping;

use ndarray::{prelude::*, DataMut};

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

    /// Performs [-(self.x, self.y) + (a, b)] <dot> other
    fn negate_add_dot(&self, a: f32, b: f32, other: &Vec2) -> f32 {
        return (a - self.x) * other.x + (b - self.y) * other.y;
    }
}

fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}
fn hash(value: u32) -> u8 {
    let key: u32 = 0x730d319b;
    let key1: u32 = 0x6373cd28;

    let mut keyed = (Wrapping(value) * Wrapping(key)).0;
    keyed ^= keyed.rotate_left(3) ^ keyed.rotate_left(17);
    keyed ^= keyed.rotate_left(4) ^ keyed.rotate_left(27) ^ key1;
    keyed ^= keyed.rotate_left(6) ^ keyed.rotate_left(21) ^ key1;

    keyed.rotate_right((value & 0xf) as u32) as u8
}

fn random_vector(row_index: usize, col_index: usize) -> Vec2 {
    let hash_value = hash(row_index as u32 | (col_index as u32).rotate_left(16));
    let direction = hash_value as f32 / 255.0;
    Vec2::from_direction(direction * std::f32::consts::PI * 2.0)
}

pub trait Perlin {
    fn perlin_inplace(&mut self, row_index: usize, col_index: usize, scale: usize);
}

impl<E, S> Perlin for ArrayBase<S, Dim<[usize; 2]>>
where
    E: From<f32> + Copy,
    S: DataMut<Elem = E>,
{
    fn perlin_inplace(&mut self, row_index: usize, col_index: usize, scale: usize) {
        let corners = [
            random_vector(row_index, col_index),
            random_vector(row_index, col_index + 1),
            random_vector(row_index + 1, col_index),
            random_vector(row_index + 1, col_index + 1),
        ];

        let vert_pixel_size = 1.0 / scale as f32;
        let horz_pixel_size = 1.0 / scale as f32;

        self.indexed_iter_mut().for_each(|((i, j), val)| {
            let pixel = Vec2::new(
                horz_pixel_size / 2.0 + horz_pixel_size * j as f32,
                vert_pixel_size / 2.0 + vert_pixel_size * i as f32,
            );
            let p1 = pixel.negate_add_dot(0.0, 0.0, &corners[0]);
            let p2 = pixel.negate_add_dot(1.0, 0.0, &corners[1]);
            let p3 = pixel.negate_add_dot(0.0, 1.0, &corners[2]);
            let p4 = pixel.negate_add_dot(1.0, 1.0, &corners[3]);

            let interp1 = interpolate(p1, p2, pixel.x);
            let interp2 = interpolate(p3, p4, pixel.x);
            let interp = interpolate(interp1, interp2, pixel.y);

            *val = interp.into();
        });
    }
}
pub trait AddPerlinNoise {
    /// Add perlin noise (in-place) to the array
    ///
    /// scale: The number of pixel between consecutive lattice
    ///        nodes used to generate the noise.
    fn add_perlin_noise(&mut self, scale: usize);
}

impl<E, S> AddPerlinNoise for ArrayBase<S, Dim<[usize; 2]>>
where
    E: From<f32> + Copy,
    S: DataMut<Elem = E>,
{
    fn add_perlin_noise(&mut self, scale: usize) {
        if let [rows, cols] = self.shape() {
            let (rows, cols) = (*rows, *cols);

            for row_index in (0..rows).step_by(scale) {
                for col_index in (0..cols).step_by(scale) {
                    let row_end = rows.min(row_index + scale);
                    let col_end = cols.min(col_index + scale);
                    let mut slice = self.slice_mut(s![row_index..row_end, col_index..col_end]);
                    slice.perlin_inplace(row_index / scale, col_index / scale, scale);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
