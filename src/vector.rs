use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Error {
    MismatchLen,
}

pub type Vector = Vec<f32>;

pub fn add(v: &Vector, w: &Vector) -> Result<Vector, Error> {
    if v.len() != w.len() {
        return Err(Error::MismatchLen);
    }

    let dim = v.len();
    let mut t = vec![0.0; dim];

    for i in 0..dim {
        t[i] = v[i] + w[i];
    }

    Ok(t)
}

pub fn sub(v: &Vector, w: &Vector) -> Result<Vector, Error> {
    if v.len() != w.len() {
        return Err(Error::MismatchLen);
    }

    let dim = v.len();
    let mut t = vec![0.0; dim];

    for i in 0..dim {
        t[i] = v[i] - w[i];
    }

    Ok(t)
}

pub fn mul(v: &Vector, s: f32) -> Vector {
    let dim = v.len();
    let mut t = vec![0.0; dim];
    for i in 0..dim {
        t[i] = v[i] * s;
    }

    t
}

#[cfg(test)]
mod tests {
    use crate::vector::{add, mul, sub, Error};

    #[test]
    fn add_error() {
        let v = vec![1.0, 2.0, 3.0];
        let w = vec![1.0, 2.0];

        let got_err = add(&v, &w).unwrap_err();
        assert_eq!(Error::MismatchLen, got_err)
    }

    #[test]
    fn add_ok() {
        let v = vec![1.0, 2.0, 3.0];
        let w = vec![1.0, 2.0, 3.0];

        let got = add(&v, &w).unwrap();
        let expect = vec![2.0, 4.0, 6.0];
        assert_eq!(expect, got)
    }

    #[test]
    fn sub_error() {
        let v = vec![1.0, 2.0, 3.0];
        let w = vec![1.0, 2.0];

        let got_err = sub(&v, &w).unwrap_err();
        assert_eq!(Error::MismatchLen, got_err)
    }

    #[test]
    fn sub_ok() {
        let v = vec![1.0, 3.0, 5.0];
        let w = vec![2.0, 2.0, 3.0];

        let got = sub(&v, &w).unwrap();
        let expect = vec![-1.0, 1.0, 2.0];
        assert_eq!(expect, got)
    }

    #[test]
    fn mul_ok() {
        let v = vec![1.0, 2.0, 3.0];
        let s = 2.0;

        let got = mul(&v, s);
        let expect = vec![2.0, 4.0, 6.0];
        assert_eq!(expect, got);
    }
}
