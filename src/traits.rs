use std::clone::Clone;
use std::marker::Copy;
use std::ops::{Add, Div, Mul, Sub};

use num_complex::Complex;
use num_traits::Zero;

pub trait Sqrt: Sized {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        (self as f32).sqrt()
    }
}

impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt()
    }
}

impl Sqrt for Complex<f32> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Sqrt for Complex<f64> {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait Num<T>:
    Add<T, Output = Self>
    + Div<T, Output = Self>
    + Mul<T, Output = Self>
    + Sub<T, Output = Self>
    + Sqrt
    + Zero
    + Clone
    + Copy
{
}

impl<T> Num<T> for T where
    T: Add<T, Output = Self>
        + Div<T, Output = Self>
        + Mul<T, Output = Self>
        + Sub<T, Output = Self>
        + Sqrt
        + Zero
        + Clone
        + Copy
{
}
