use std::{num::Wrapping, ops::Add};

use ndarray::{prelude::*, DataMut};

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

fn random_vector(x: usize, y: usize) -> Vec2 {
    let hash_value = hash(x as u32 | (y as u32).rotate_left(16));
    let direction = hash_value as f32 / 255.0;
    Vec2::from_direction(direction * std::f32::consts::PI * 2.0)
}

trait PerlinNoiseSquare {
    /// Insrt a single square of perlin noise.
    fn perlin_noise_square(&mut self, x_offset: usize, y_offset: usize, scale: usize, amp: f32);
}

impl<E, S> PerlinNoiseSquare for ArrayBase<S, Dim<[usize; 2]>>
where
    E: From<f32> + Add<Output = E> + Copy,
    S: DataMut<Elem = E>,
{
    fn perlin_noise_square(&mut self, x_offset: usize, y_offset: usize, scale: usize, amp: f32) {
        let corners = [
            random_vector(x_offset, y_offset),
            random_vector(x_offset, y_offset + 1),
            random_vector(x_offset + 1, y_offset),
            random_vector(x_offset + 1, y_offset + 1),
        ];

        let vert_pixel_size = 1.0 / scale as f32;
        let horz_pixel_size = 1.0 / scale as f32;

        self.indexed_iter_mut().for_each(|((i, j), val)| {
            let pixel = Vec2::new(
                horz_pixel_size / 2.0 + horz_pixel_size * j as f32,
                vert_pixel_size / 2.0 + vert_pixel_size * i as f32,
            );
            let p1 = pixel.negate_add_dot(0.0, 0.0, &corners[0]);
            let p2 = pixel.negate_add_dot(0.0, 1.0, &corners[1]);
            let p3 = pixel.negate_add_dot(1.0, 0.0, &corners[2]);
            let p4 = pixel.negate_add_dot(1.0, 1.0, &corners[3]);

            let interp1 = interpolate(p1, p2, pixel.y);
            let interp2 = interpolate(p3, p4, pixel.y);
            let interp = interpolate(interp1, interp2, pixel.x);

            *val = *val + (amp * interp / 2.0).into();
        });
    }
}
pub trait AddPerlinNoise {
    /// Add perlin noise (in-place) to the array
    ///
    /// scale: The number of pixel between consecutive lattice
    ///        nodes used to generate the noise.
    /// amp: Amplitude of the noise (difference between min and max peaks)\
    fn add_perlin_noise(&mut self, scale: usize, amp: f32);
}

impl<E, S> AddPerlinNoise for ArrayBase<S, Dim<[usize; 2]>>
where
    E: From<f32> + Add<Output = E> + Copy,
    S: DataMut<Elem = E>,
{
    fn add_perlin_noise(&mut self, scale: usize, amp: f32) {
        if let [rows, cols] = self.shape() {
            let (rows, cols) = (*rows, *cols);

            for row_index in (0..rows).step_by(scale) {
                for col_index in (0..cols).step_by(scale) {
                    let row_end = rows.min(row_index + scale);
                    let col_end = cols.min(col_index + scale);
                    let mut slice = self.slice_mut(s![row_index..row_end, col_index..col_end]);
                    slice.perlin_noise_square(col_index / scale, row_index / scale, scale, amp);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {}
