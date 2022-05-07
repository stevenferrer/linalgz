use std::ops::{Add, Div, Mul, Sub};

use crate::{Num, Vector};

pub fn dot<T: Num>(v: &Vector<T>, w: &Vector<T>) -> T {
    assert_len(v.0.len(), w.0.len());

    let mut prod = T::zero();
    let dim = v.0.len();
    for i in 0..dim {
        prod = prod + v.0[i] * w.0[i];
    }

    prod
}

pub fn norm<T: Num>(v: &Vector<T>) -> T {
    let mut prod = T::zero();
    for e in v.0.iter() {
        prod = prod + *e * *e;
    }

    prod.sqrt()
}

pub fn outer<T: Num>(v: &Vector<T>, w: &Vector<T>) -> Vector<Vector<T>> {
    let mut mat = Vector(Vec::with_capacity(v.0.len()));

    for ve in v.0.iter() {
        let mut row = Vec::with_capacity(w.0.len());
        for we in w.0.iter() {
            row.push(*ve * *we);
        }
        mat.0.push(Vector(row))
    }

    mat
}

pub fn cross<T: Num>(v: &Vector<T>, w: &Vector<T>) -> Vector<T> {
    assert_len(3, v.0.len());

    let x = v.0[1] * w.0[2] - v.0[2] * w.0[1];
    let y = v.0[2] * w.0[0] - v.0[0] * w.0[2];
    let z = v.0[0] * w.0[1] - v.0[1] * w.0[0];

    Vector(vec![x, y, z])
}

pub fn unit_vector<T: Num>(v: &Vector<T>) -> Vector<T> {
    v / norm(v)
}

impl<T: Num> Add for Vector<T> {
    type Output = Vector<T>;

    fn add(self, w: Self) -> Self::Output {
        assert_len(self.0.len(), w.0.len());

        let dim = self.0.len();
        let mut t = vec![T::zero(); dim];

        for i in 0..dim {
            t[i] = self.0[i] + w.0[i];
        }

        Vector(t)
    }
}

impl<T: Num> Sub for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, w: Self) -> Self::Output {
        assert_len(self.0.len(), w.0.len());

        let dim = self.0.len();
        let mut t = vec![T::zero(); dim];
        for i in 0..dim {
            t[i] = self.0[i] - w.0[i];
        }

        Vector(t)
    }
}

impl<T: Num> Div<T> for &Vector<T> {
    type Output = Vector<T>;

    fn div(self, s: T) -> Self::Output {
        let dim = self.0.len();
        let mut t = vec![T::zero(); dim];
        for i in 0..dim {
            t[i] = self.0[i] / s;
        }

        Vector(t)
    }
}

impl<T: Num> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, s: T) -> Self::Output {
        &self / s
    }
}

impl<T: Num> Mul<T> for &Vector<T> {
    type Output = Vector<T>;

    fn mul(self, s: T) -> Self::Output {
        let dim = self.0.len();
        let mut t = vec![T::zero(); dim];
        for i in 0..dim {
            t[i] = self.0[i] * s;
        }

        Vector(t)
    }
}

impl<T: Num> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, s: T) -> Self::Output {
        &self * s
    }
}

impl<T: Num> Mul for &Vector<T> {
    type Output = Vector<T>;

    fn mul(self, w: Self) -> Self::Output {
        assert_len(self.0.len(), w.0.len());

        let dim = self.0.len();
        let mut t = vec![T::zero(); dim];
        for i in 0..dim {
            t[i] = self.0[i] * w.0[i];
        }

        Vector(t)
    }
}

impl<T: Num> Mul for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, w: Self) -> Self::Output {
        &self * &w
    }
}

fn assert_len(expect: usize, got: usize) {
    if expect != got {
        panic!("invalid vector length")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_ok() {
        let v = Vector(vec![1., 2., 3.]);
        let w = Vector(vec![1., 2., 6.]);

        let got = v + w;
        let expect = Vector(vec![2., 4., 9.]);
        assert_eq!(expect, got)
    }

    #[test]
    fn sub_ok() {
        let v = Vector(vec![1., 2., 8.]);
        let w = Vector(vec![2., 3.5, 5.]);

        let got = v - w;
        let expect = Vector(vec![-1., -1.5, 3.]);
        assert_eq!(expect, got)
    }

    #[test]
    fn div_ok() {
        let v = Vector(vec![1., 3., 5.]);
        let s = 2.0;

        let got = v / s;
        let expect = Vector(vec![0.5, 1.5, 2.5]);
        assert_eq!(expect, got)
    }

    #[test]
    fn mul_ok() {
        let v = Vector(vec![1., 2., 3.]);
        let s = 2.;

        let got = v * s;
        let expect = Vector(vec![2., 4., 6.]);
        assert_eq!(expect, got);
    }

    #[test]
    fn dot_ok() {
        let v = Vector(vec![1., 2., 3.]);
        let w = Vector(vec![1., 2., 3.]);

        let prod = dot(&v, &w);
        assert_eq!(14., prod)
    }

    #[test]
    fn norm_ok() {
        let v: Vector<f32> = Vector(vec![1., 2., 3.]);

        let mag = norm(&v);
        assert_eq!(3.7416575, mag);
    }

    #[test]
    fn hadamard_ok() {
        let v = Vector(vec![1., 3., 5.]);
        let w = Vector(vec![2., 2., 3.]);

        // element-wise multiplication
        let got = v * w;
        let expect = Vector(vec![2., 6., 15.]);
        assert_eq!(expect, got)
    }

    #[test]
    fn outer_ok() {
        let v: Vector<f32> = Vector(vec![1., 2., 3.]);
        let w: Vector<f32> = Vector(vec![2., 4., 6.]);

        let got = outer(&v, &w);
        let expect = Vector(vec![
            Vector(vec![2., 4., 6.]),
            Vector(vec![4., 8., 12.]),
            Vector(vec![6., 12., 18.]),
        ]);

        assert_eq!(expect, got)
    }

    #[test]
    fn cross_ok() {
        let v = Vector(vec![1., 2., 3.]);
        let w = Vector(vec![4., 5., 6.]);

        let got = cross(&v, &w);
        let expect = Vector(vec![-3., 6., -3.]);
        assert_eq!(expect, got)
    }

    #[test]
    fn unit_vector_ok() {
        let v: Vector<f32> = Vector(vec![1., 2., 3.]);

        let got = unit_vector(&v);
        let expect = Vector(vec![0.26726124, 0.5345225, 0.8017837]);
        assert_eq!(expect, got);

        assert_eq!(1., norm(&got).ceil());
    }
}
