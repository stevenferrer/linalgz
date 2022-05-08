use std::ops::{Add, Div, Mul, Sub};

use crate::traits::Num;
use crate::utils::assert_len;
use crate::vector::Vector;

impl<T: Num<T>> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, w: Self) -> Self::Output {
        assert_len(self.0.len(), w.0.len());

        let dim = self.0.len();
        let mut t = Vec::with_capacity(dim);

        for i in 0..dim {
            t.push(self.0[i] + w.0[i])
        }

        Vector(t)
    }
}

impl<T: Num<T>> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, w: Self) -> Self::Output {
        assert_len(self.0.len(), w.0.len());

        let dim = self.0.len();
        let mut t = Vec::with_capacity(dim);
        for i in 0..dim {
            t.push(self.0[i] - w.0[i]);
        }

        Vector(t)
    }
}

impl<T: Num<T>> Div<T> for &Vector<T> {
    type Output = Vector<T>;

    fn div(self, s: T) -> Self::Output {
        let dim = self.0.len();
        let mut t = Vec::with_capacity(dim);
        for i in 0..dim {
            t.push(self.0[i] / s);
        }

        Vector(t)
    }
}

impl<T: Num<T>> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, s: T) -> Self::Output {
        &self / s
    }
}

impl<T: Num<T>> Mul<T> for &Vector<T> {
    type Output = Vector<T>;

    fn mul(self, s: T) -> Self::Output {
        let dim = self.0.len();
        let mut t = Vec::with_capacity(dim);
        for i in 0..dim {
            t.push(self.0[i] * s);
        }

        Vector(t)
    }
}

impl<T: Num<T>> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, s: T) -> Self::Output {
        &self * s
    }
}

impl<T: Num<T>> Mul for &Vector<T> {
    type Output = Vector<T>;

    fn mul(self, w: Self) -> Self::Output {
        assert_len(self.0.len(), w.0.len());

        let dim = self.0.len();
        let mut t = Vec::with_capacity(dim);
        for i in 0..dim {
            t.push(self.0[i] * w.0[i]);
        }

        Vector(t)
    }
}

impl<T: Num<T>> Mul for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, w: Self) -> Self::Output {
        &self * &w
    }
}
