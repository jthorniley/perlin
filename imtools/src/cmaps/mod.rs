use crate::image_types::{RgbaImage, ScalarImageView, ScalarPixel};
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgb, Pixel};

pub trait CMap<'a, Pixel, Image>
where
    Pixel: 'a + ScalarPixel,
    Image: ScalarImageView<'a, Pixel>,
{
    type Output: RgbaImage;

    /// Convert to an RGBA image
    fn cmap(&'a self, input: Image) -> Self::Output;
}

pub struct Grayscale;

impl<'a, Pixel, Image> CMap<'a, Pixel, Image> for Grayscale
where
    Pixel: 'a + ScalarPixel,
    Image: ScalarImageView<'a, Pixel>,
{
    type Output = Array3<u8>;

    fn cmap(&'a self, input: Image) -> Self::Output {
        let input_view: ArrayView2<'a, Pixel> = input.into();
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

pub struct GradientCMap<T>
where
    T: AsRef<[(f32, LinSrgb)]>,
{
    gradient: Gradient<LinSrgb, T>,
}

impl<T> GradientCMap<T>
where
    T: AsRef<[(f32, LinSrgb)]>,
{
    pub fn new(gradient: Gradient<LinSrgb, T>) -> GradientCMap<T> {
        GradientCMap { gradient }
    }
}

impl<'a, T, Pixel, I> CMap<'a, Pixel, I> for GradientCMap<T>
where
    Pixel: 'a + ScalarPixel,
    T: AsRef<[(f32, LinSrgb)]>,
    I: ScalarImageView<'a, Pixel>,
{
    type Output = Array3<u8>;

    fn cmap(&'a self, input: I) -> Self::Output {
        let input_view: ArrayView2<'a, Pixel> = input.into();

        let input_min = input_view.min().unwrap().to_f32().unwrap();
        let input_max = input_view.max().unwrap().to_f32().unwrap();
        let input_range = input_max - input_min;

        let (grad_min, grad_max) = self.gradient.domain();
        let grad_range = grad_max - grad_min;

        let mut output = Array::zeros((input_view.shape()[0], input_view.shape()[1], 3));
        azip!((mut o in output.lanes_mut(Axis(2)), &i in input_view) {
            let x = (i.to_f32().unwrap() - input_min) / input_range;
            let x = (x * grad_range) + grad_min;
            let value: [u8; 3] = self.gradient.get(x).into_format().into_raw();
            o.assign(&Array1::from_iter(value));
        });
        output
    }
}
