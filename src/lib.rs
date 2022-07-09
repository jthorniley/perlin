use std::{num::Wrapping, ops::Range};

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

fn random_vector(x: u32, y: u32) -> Vec2 {
    let hash_value = hash(x | y.rotate_left(16));
    let direction = hash_value as f32 / 255.0;
    Vec2::from_direction(direction * std::f32::consts::PI * 2.0)
}

pub trait Perlin {
    fn perlin_inplace(&mut self, x: u32, y: u32);
}

impl<E, S> Perlin for ArrayBase<S, Dim<[usize; 2]>>
where
    E: From<f32> + Copy,
    S: DataMut<Elem = E>,
{
    fn perlin_inplace(&mut self, x: u32, y: u32) {
        let corners = [
            random_vector(x, y),
            random_vector(x, y + 1),
            random_vector(x + 1, y),
            random_vector(x + 1, y + 1),
        ];

        if let [nrows, ncols] = self.shape() {
            let vert_pixel_size = 1.0 / *nrows as f32;
            let horz_pixel_size = 1.0 / *ncols as f32;

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

                *val = interp.into();
            });
        }
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
        if let [ny, nx] = self.shape() {
            let (nx, ny) = (*nx, *ny);
            let x_iter = IntervalIter::new(scale, nx);

            for (x, x_range) in x_iter {
                let y_iter = IntervalIter::new(scale, ny);
                for (y, y_range) in y_iter {
                    let slice_info = s![y_range, x_range.clone()];
                    let mut slice = self.slice_mut(slice_info);
                    slice.perlin_inplace(x as u32, y as u32);
                }
            }
        }
    }
}

struct IntervalIter {
    n: u32,
    pos: usize,
    scale: usize,
    len: usize,
}

impl IntervalIter {
    fn new(scale: usize, len: usize) -> IntervalIter {
        IntervalIter {
            n: 0,
            pos: 0,
            scale,
            len,
        }
    }
}

impl Iterator for IntervalIter {
    type Item = (u32, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pos;
        let end = (self.pos + self.scale).min(self.len);

        if start >= end {
            None
        } else {
            self.pos += self.scale;
            self.n += 1;
            Some((self.n - 1, Range { start, end }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_iter_behaviour() {
        let interval_iter = IntervalIter::new(10, 25);
        let collected: Vec<Range<usize>> = interval_iter.map(|(n, range)| range).collect();
        assert_eq!(collected, vec![0..10, 10..20, 20..25]);
    }

    #[test]
    fn test_interval_iter_behaviour_short() {
        let interval_iter = IntervalIter::new(10, 2);
        let collected: Vec<Range<usize>> = interval_iter.map(|(n, range)| range).collect();
        assert_eq!(collected, vec![0..2]);
    }
    #[test]
    fn test_interval_iter_behaviour_len_zero() {
        let interval_iter = IntervalIter::new(10, 0);
        let collected: Vec<Range<usize>> = interval_iter.map(|(n, range)| range).collect();
        assert_eq!(collected, vec![]);
    }
    #[test]
    fn test_interval_iter_behaviour_len_exact() {
        let interval_iter = IntervalIter::new(4, 12);
        let collected: Vec<Range<usize>> = interval_iter.map(|(n, range)| range).collect();
        assert_eq!(collected, vec![0..4, 4..8, 8..12]);
    }
    #[test]
    fn test_interval_iter_behaviour_scale_zero() {
        let interval_iter = IntervalIter::new(0, 12);
        let collected: Vec<Range<usize>> = interval_iter.map(|(n, range)| range).collect();
        assert_eq!(collected, vec![]);
    }
}
