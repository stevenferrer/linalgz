use std::fmt;
use std::ops::Mul;

#[derive(PartialEq, Debug, Clone)]
pub enum Error {
    LenMismatch,
}

pub type Vector = Vec<f32>;

// TODO: define macro to assert length

pub fn add(v: &Vector, w: &Vector) -> Result<Vector, Error> {
    if v.len() != w.len() {
        return Err(Error::LenMismatch);
    }

    let dim = v.len();
    let mut t = vec![0.; dim];

    for i in 0..dim {
        t[i] = v[i] + w[i];
    }

    Ok(t)
}

pub fn sub(v: &Vector, w: &Vector) -> Result<Vector, Error> {
    if v.len() != w.len() {
        return Err(Error::LenMismatch);
    }

    let dim = v.len();
    let mut t = vec![0.; dim];

    for i in 0..dim {
        t[i] = v[i] - w[i];
    }

    Ok(t)
}

pub fn mul(v: &Vector, s: f32) -> Vector {
    let dim = v.len();
    let mut t = vec![0.; dim];
    for i in 0..dim {
        t[i] = v[i] * s;
    }

    t
}

pub fn dot(v: &Vector, w: &Vector) -> Result<f32, Error> {
    if v.len() != w.len() {
        return Err(Error::LenMismatch);
    }

    let mut prod = 0.;
    let dim = v.len();
    for i in 0..dim {
        prod += v[i] * w[i];
    }

    Ok(prod)
}

pub fn norm(v: &Vector) -> f32 {
    let mut prod = 0.;
    for e in v.iter() {
        prod += e * e;
    }

    prod.sqrt()
}

pub fn e_mul(v: &Vector, w: &Vector) -> Result<Vector, Error> {
    if v.len() != w.len() {
        return Err(Error::LenMismatch);
    }

    let dim = v.len();
    let mut t = vec![0.; dim];

    for i in 0..dim {
        t[i] = v[i] * w[i];
    }

    Ok(t)
}

pub fn outer(v: &Vector, w: &Vector) -> Vec<Vec<f32>> {
    let mut mat: Vec<Vec<f32>> = Vec::with_capacity(v.len());

    for ve in v.iter() {
        let mut row: Vec<f32> = Vec::with_capacity(w.len());
        for we in w.iter() {
            row.push(ve * we);
        }
        mat.push(row)
    }

    mat
}

#[cfg(test)]
mod tests {
    use crate::vector::{add, dot, e_mul, mul, norm, outer, sub, Error};

    #[test]
    fn add_error() {
        let v = vec![1., 2., 3.];
        let w = vec![1., 2.];

        let got_err = add(&v, &w).unwrap_err();
        assert_eq!(Error::LenMismatch, got_err)
    }

    #[test]
    fn add_ok() {
        let v = vec![1., 2., 3.];
        let w = vec![1., 2., 3.];

        let got = add(&v, &w).unwrap();
        let expect = vec![2., 4., 6.];
        assert_eq!(expect, got)
    }

    #[test]
    fn sub_error() {
        let v = vec![1., 2., 3.];
        let w = vec![1., 2.];

        let got_err = sub(&v, &w).unwrap_err();
        assert_eq!(Error::LenMismatch, got_err)
    }

    #[test]
    fn sub_ok() {
        let v = vec![1., 3., 5.];
        let w = vec![2., 2., 3.];

        let got = sub(&v, &w).unwrap();
        let expect = vec![-1., 1., 2.];
        assert_eq!(expect, got)
    }

    #[test]
    fn mul_ok() {
        let v = vec![1., 2., 3.];
        let s = 2.;

        let got = mul(&v, s);
        let expect = vec![2., 4., 6.];
        assert_eq!(expect, got);
    }

    #[test]
    fn dot_err() {
        let v = vec![1., 2., 3.];
        let w = vec![1., 2.];

        let got_err = dot(&v, &w).unwrap_err();
        assert_eq!(Error::LenMismatch, got_err)
    }

    #[test]
    fn dot_ok() {
        let v = vec![1., 2., 3.];
        let w = vec![1., 2., 3.];

        let prod = dot(&v, &w).unwrap();
        assert_eq!(14., prod)
    }

    #[test]
    fn norm_ok() {
        let v = vec![1., 2., 3.];

        let mag = norm(&v);
        assert_eq!(3.7416575, mag);
    }

    #[test]
    fn ew_mul_ok() {
        let v = vec![1., 3., 5.];
        let w = vec![2., 2., 3.];

        let got = e_mul(&v, &w).unwrap();
        let expect = vec![2., 6., 15.];
        assert_eq!(expect, got)
    }

    #[test]
    fn outer_ok() {
        let v = vec![1., 2., 3.];
        let w = vec![2., 4., 6.];

        let got = outer(&v, &w);

        let expect = vec![vec![2., 4., 6.], vec![4., 8., 12.], vec![6., 12., 18.]];

        assert_eq!(expect, got)
    }
}
