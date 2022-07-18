use std::borrow::Borrow;

use ndarray::{Array3, ArrayView2, ArrayViewMut2};
use num_traits::{Num, NumAssign, NumCast};

pub trait ScalarPixel: Num + NumCast + NumAssign + PartialOrd + Copy {}
impl<T> ScalarPixel for T where T: Num + NumCast + NumAssign + PartialOrd + Copy {}

pub trait ScalarImageView<'a, T: 'a + ScalarPixel>: Into<ArrayView2<'a, T>> {}
impl<'a, I, T: 'a + ScalarPixel> ScalarImageView<'a, T> for I where I: Into<ArrayView2<'a, T>> {}

pub trait ScalarImageViewMut<'a, T: 'a + ScalarPixel>: Into<ArrayViewMut2<'a, T>> {}
impl<'a, I, T: 'a + ScalarPixel> ScalarImageViewMut<'a, T> for I where I: Into<ArrayViewMut2<'a, T>> {}

pub trait RgbaImage: Borrow<Array3<u8>> {}
impl<I> RgbaImage for I where I: Borrow<Array3<u8>> {}
