use ndarray::prelude::*;

pub trait AsFlatSlice {
    type T;
    fn as_flat_slice(&self) -> &[Self::T];
}

impl<T, D: Dimension> AsFlatSlice for Array<[T; 4], D> {
    type T = T;

    fn as_flat_slice(&self) -> &[Self::T] {
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const Self::T, self.len() * 4) }
    }
}
