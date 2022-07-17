use std::ops::{Add, Div, Sub};

use ndarray::{azip, Array, ArrayBase, Axis, Data, Dim, Zip};
use ndarray_stats::QuantileExt;

pub trait MapImpl {
    /// Convert to an RGBA image
    fn map_impl<E, S>(&self, input: &ArrayBase<S, Dim<[usize; 2]>>) -> Array<u8, Dim<[usize; 3]>>
    where
        S: Data<Elem = E>,
        E: Into<f32>
            + Sub<E, Output = E>
            + Add<E, Output = E>
            + Div<E, Output = E>
            + PartialOrd
            + Copy;
}

pub struct Grayscale;

impl MapImpl for Grayscale {
    fn map_impl<E, S>(&self, input: &ArrayBase<S, Dim<[usize; 2]>>) -> Array<u8, Dim<[usize; 3]>>
    where
        S: Data<Elem = E>,
        E: Into<f32>
            + Sub<E, Output = E>
            + Add<E, Output = E>
            + Div<E, Output = E>
            + PartialOrd
            + Copy,
    {
        let min_value = input.min().unwrap();
        let max_value = input.max().unwrap();
        let range = *max_value - *min_value;
        let mut output = Array::zeros((input.shape()[0], input.shape()[1], 4));
        azip!((mut o in output.lanes_mut(Axis(2)), &i in input) {
            let level = (255.0f32 * ((i - *min_value) / range).into()) as u8;
            o[0] = level;
            o[1] = level;
            o[2] = level;
            o[3] = 255;
        });
        output
    }
}

pub trait MapToRgba {
    fn map_to_rgba(&self, map_impl: &impl MapImpl) -> Array<u8, Dim<[usize; 3]>>;
}

impl<E, S> MapToRgba for ArrayBase<S, Dim<[usize; 2]>>
where
    E: Into<f32> + Sub<E, Output = E> + Add<E, Output = E> + Div<E, Output = E> + PartialOrd + Copy,
    S: Data<Elem = E>,
{
    fn map_to_rgba(&self, map_impl: &impl MapImpl) -> Array<u8, Dim<[usize; 3]>> {
        map_impl.map_impl(self)
    }
}
