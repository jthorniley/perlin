use ndarray::{azip, Array, ArrayView2, Axis, Dim};
use ndarray_stats::QuantileExt;

use crate::image_types::{ScalarImage, ScalarPixel};

pub trait CMap {
    /// Convert to an RGBA image
    fn cmap<'a, T, I>(&'a self, input: I) -> Array<u8, Dim<[usize; 3]>>
    where
        T: 'a + ScalarPixel,
        I: ScalarImage<'a, T>;
}

pub struct Grayscale;

impl CMap for Grayscale {
    fn cmap<'a, T, I>(&'a self, input: I) -> Array<u8, Dim<[usize; 3]>>
    where
        T: 'a + ScalarPixel,
        I: ScalarImage<'a, T>,
    {
        let input_view: ArrayView2<'a, T> = input.into();
        let min_value = input_view.min().unwrap();
        let max_value = input_view.max().unwrap();
        let range = *max_value - *min_value;
        let mut output = Array::zeros((input_view.shape()[0], input_view.shape()[1], 4));
        azip!((mut o in output.lanes_mut(Axis(2)), &i in input_view) {
            let scaled= (i - *min_value) / range;
            let level = (scaled.to_f32().unwrap() * 255.0f32) as u8;
            o[0] = level;
            o[1] = level;
            o[2] = level;
            o[3] = 255u8;
        });
        output
    }
}
