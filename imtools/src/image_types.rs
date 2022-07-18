use num_traits::{Num, NumAssign, NumCast};

pub trait Pixel: Num + NumCast + NumAssign + PartialOrd + Copy {}

impl<T> Pixel for T where T: Num + NumCast + NumAssign + PartialOrd + Copy {}
