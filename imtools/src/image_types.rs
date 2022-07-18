use ndarray::{ArrayView2, ArrayViewMut2};
use num_traits::{Num, NumAssign, NumCast};

pub trait ScalarPixel: Num + NumCast + NumAssign + PartialOrd + Copy {}

impl<T> ScalarPixel for T where T: Num + NumCast + NumAssign + PartialOrd + Copy {}

pub trait ScalarImage<'a, T: 'a + ScalarPixel>: Into<ArrayView2<'a, T>> {}

impl<'a, I, T: 'a + ScalarPixel> ScalarImage<'a, T> for I where I: Into<ArrayView2<'a, T>> {}

pub trait ScalarImageMut<'a, T: 'a + ScalarPixel>: Into<ArrayViewMut2<'a, T>> {}

impl<'a, I, T: 'a + ScalarPixel> ScalarImageMut<'a, T> for I where I: Into<ArrayViewMut2<'a, T>> {}
