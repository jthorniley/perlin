use std::num::Wrapping;

use ndarray::prelude::*;

use crate::image_types::{ScalarImageViewMut, ScalarPixel};

/// Interpolate between two values according to a weight.
///
/// Calculates a value somewhere between `a0` and `a1`,
/// according to the value of `w`. If `w` is close to 1, the
/// result is close to `a1`, if `w` is close to 0, the result
/// is close to `a0`.
///
/// This uses the 5th order [smoothstep] function.
///
/// [smoothstep]: https://en.wikipedia.org/wiki/Smoothstep#5th-order_equation
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
/// This is a cheap rotate-xor-prime hash function as described
/// by this [web page].
///
/// [web page]: https://www.gkbrk.com/wiki/avalanche-diagram/
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

pub struct Perlin<T: ScalarPixel> {
    scale: usize,
    amp: T,
}

impl<T: ScalarPixel> Perlin<T> {
    pub fn new(scale: usize, amp: T) -> Perlin<T> {
        Perlin { scale, amp }
    }

    pub fn add_to_image<'a>(&'a self, image: impl ScalarImageViewMut<'a, T>) {
        let mut view: ArrayViewMut2<'a, T> = image.into();
        if let [rows, cols] = view.shape() {
            let (rows, cols) = (*rows, *cols);
            let mut vertex = Vertex::new(0, 0);

            for row_index in (0..rows).step_by(self.scale) {
                for col_index in (0..cols).step_by(self.scale) {
                    let row_end = rows.min(row_index + self.scale);
                    let col_end = cols.min(col_index + self.scale);
                    let mut slice = view.slice_mut(s![row_index..row_end, col_index..col_end]);
                    self.perlin_noise_square(&vertex, &mut slice);
                    vertex = Vertex::new(vertex.i0 + 1, vertex.i1);
                }
                vertex = Vertex::new(0, vertex.i1 + 1);
            }
        }
    }

    fn perlin_noise_square<'a>(&'a self, vertex: &Vertex, slice: &mut ArrayViewMut2<'a, T>) {
        let corners = [
            vertex.vec(),
            Vertex::new(vertex.i0, vertex.i1 + 1).vec(),
            Vertex::new(vertex.i0 + 1, vertex.i1).vec(),
            Vertex::new(vertex.i0 + 1, vertex.i1 + 1).vec(),
        ];

        let pixel_size = 1.0f32 / (self.scale as f32);
        let half_pixel_size = pixel_size / 2.0f32;

        slice.indexed_iter_mut().for_each(|((i, j), val)| {
            let pixel = Vec2::new(
                half_pixel_size + pixel_size * (j as f32),
                half_pixel_size + pixel_size * (i as f32),
            );
            let p1 = pixel.negate_add_dot(0.0f32, 0.0f32, &corners[0]);
            let p2 = pixel.negate_add_dot(0.0f32, 1.0f32, &corners[1]);
            let p3 = pixel.negate_add_dot(1.0f32, 0.0f32, &corners[2]);
            let p4 = pixel.negate_add_dot(1.0f32, 1.0f32, &corners[3]);

            let interp1 = interpolate(p1, p2, pixel.y);
            let interp2 = interpolate(p3, p4, pixel.y);
            let interp = interpolate(interp1, interp2, pixel.x);

            *val = *val + (self.amp * T::from(interp / 2.0f32).unwrap());
        })
    }
}
