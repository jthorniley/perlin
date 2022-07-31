use crate::image_types::{RgbaImageArray, ScalarImageView, ScalarPixel};
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;
use palette::{Gradient, LinSrgb, Pixel};

pub trait CMap<'a, Pixel, Image>
where
    Pixel: 'a + ScalarPixel,
    Image: ScalarImageView<'a, Pixel>,
{
    type Output: RgbaImageArray;

    /// Convert to an RGB image
    fn cmap(&'a self, input: Image, output: &mut Self::Output);
}

pub struct Grayscale;

impl<'a, Pixel, Image> CMap<'a, Pixel, Image> for Grayscale
where
    Pixel: 'a + ScalarPixel,
    Image: ScalarImageView<'a, Pixel>,
{
    type Output = Array2<[u8; 4]>;

    fn cmap(&'a self, input: Image, output: &mut Self::Output) {
        let input_view: ArrayView2<'a, Pixel> = input.into();
        let min_value = input_view.min().unwrap();
        let max_value = input_view.max().unwrap();
        let range = *max_value - *min_value;
        azip!((o in output, &i in input_view) {
            let scaled= (i - *min_value) / range;
            let level = (scaled.to_f32().unwrap() * 255.0f32) as u8;
            *o = [level, level, level, 255];
        });
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
    type Output = Array2<[u8; 4]>;

    fn cmap(&'a self, input: I, output: &mut Self::Output) {
        let input_view: ArrayView2<'a, Pixel> = input.into();

        let input_min = input_view.min().unwrap().to_f32().unwrap();
        let input_max = input_view.max().unwrap().to_f32().unwrap();
        let input_range = input_max - input_min;

        let (grad_min, grad_max) = self.gradient.domain();
        let grad_range = grad_max - grad_min;

        azip!((o in output, &i in input_view) {
            let x = (i.to_f32().unwrap() - input_min) / input_range;
            let x = (x * grad_range) + grad_min;
            let rgb: [u8; 3] = self.gradient.get(x).into_format().into_raw();
            *o = [rgb[0], rgb[1], rgb[2], 255];
        });
    }
}
