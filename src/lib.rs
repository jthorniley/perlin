use std::{num::Wrapping, ops::Add};

use ndarray::{prelude::*, DataMut};

/// Interpolate between two values according to a weight.
///
/// Calculates a value somewhere inbetween `a0` and `a1`,
/// according to the value of `w`. If `w` is close to 1, the
/// result is close to `a1`, if `w` is close to 0, the result
/// is close to `a0`.
///
/// This uses the 5th order smoothstep function:
/// https://en.wikipedia.org/wiki/Smoothstep#5th-order_equation
fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}

/// Calculate a hash value.
///
/// Map from the input value to an output value such that the
/// output value is "random" (changes unpredictably beetween
/// consecutive inputs) but deterministic (calling this function
/// with the same input always returns the same result).
///
/// This is a cheap rotate-xor-prime hash function from here:
///
/// https://www.gkbrk.com/wiki/avalanche-diagram/
fn hash(value: u32) -> u8 {
    let mut value = Wrapping(value);
    value *= Wrapping(7919u32);
    value ^= Wrapping(value.0.rotate_left(7u32));
    value *= Wrapping(7723u32);
    value ^= Wrapping(value.0.rotate_left(11u32));
    value *= Wrapping(7561u32);
    value ^= Wrapping(value.0.rotate_left(13u32));
    value.0 as u8
}

/// Struct for basic 2d vector manipulation.
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    /// Create a 2d vector from literal values.
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    /// Create a unit length vector from a given direction.
    fn from_direction(direction: f32) -> Vec2 {
        let (x, y) = direction.sin_cos();
        Vec2 { x, y }
    }

    /// Performs [-(self.x, self.y) + (a, b)] <dot> other
    fn negate_add_dot(&self, a: f32, b: f32, other: &Vec2) -> f32 {
        return (a - self.x) * other.x + (b - self.y) * other.y;
    }
}

/// Vertex location on the lattice used to generate the Perlin noise
struct Vertex {
    i0: usize,
    i1: usize,
}

impl Vertex {
    fn new(i0: usize, i1: usize) -> Vertex {
        Vertex { i0, i1 }
    }

    /// Hash value corresponding to the vertex
    fn hash(&self) -> u8 {
        hash(self.i0 as u32 | (self.i1 as u32).rotate_left(16))
    }

    /// Direction vector for the vertex
    fn vec(&self) -> Vec2 {
        let direction = self.hash() as f32 / 255.0;
        Vec2::from_direction(direction * std::f32::consts::PI * 2.0)
    }
}

trait PerlinNoiseSquare {
    /// Add a single lattice square of perlin noise to this array view.
    ///
    /// One corner of the square is the supplied `vertex`, the adjacent
    /// horizontal, vertical and diagonal vertices form a square.
    ///
    /// The supplied `scale` indicates how many interpolated data points
    /// are inserted between the vertices of the lattice.
    ///
    /// The `amp` (amplitude) is the max-min noise signal size (i.e. each data
    /// point will get a noise value somewhere between -amp/2 and amp/2).
    fn perlin_noise_square(&mut self, vertex: &Vertex, scale: usize, amp: f32);
}

impl<E, S> PerlinNoiseSquare for ArrayBase<S, Dim<[usize; 2]>>
where
    E: From<f32> + Add<Output = E> + Copy,
    S: DataMut<Elem = E>,
{
    fn perlin_noise_square(&mut self, vertex: &Vertex, scale: usize, amp: f32) {
        let corners = [
            vertex.vec(),
            Vertex::new(vertex.i0, vertex.i1 + 1).vec(),
            Vertex::new(vertex.i0 + 1, vertex.i1).vec(),
            Vertex::new(vertex.i0 + 1, vertex.i1 + 1).vec(),
        ];

        let pixel_size = 1.0 / scale as f32;

        self.indexed_iter_mut().for_each(|((i, j), val)| {
            let pixel = Vec2::new(
                pixel_size / 2.0 + pixel_size * j as f32,
                pixel_size / 2.0 + pixel_size * i as f32,
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
    /// Add perlin noise (in-place) to this array.
    ///
    /// This constructs a lattice over the array with the
    /// vertices `scale` data points apart. The `amp` parameter
    /// specifies the magnitude of the noise values added to the
    /// array.
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
            let mut vertex = Vertex::new(0, 0);

            for row_index in (0..rows).step_by(scale) {
                for col_index in (0..cols).step_by(scale) {
                    let row_end = rows.min(row_index + scale);
                    let col_end = cols.min(col_index + scale);
                    let mut slice = self.slice_mut(s![row_index..row_end, col_index..col_end]);
                    slice.perlin_noise_square(&vertex, scale, amp);
                    vertex = Vertex::new(vertex.i0 + 1, vertex.i1);
                }
                vertex = Vertex::new(0, vertex.i1 + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {}
