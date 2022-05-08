mod ops;

#[cfg(test)]
mod test;

use std::slice::Iter;

use crate::traits::Num;
use crate::utils::assert_len;

#[derive(PartialEq, Debug, Clone)]
pub struct Vector<T>(Vec<T>);

impl<T: Num<T>> Vector<T> {
    fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }
}

pub fn norm<T: Num<T>>(v: &Vector<T>) -> T {
    let mut p = T::zero();
    for e in v.iter() {
        p = p + *e * *e;
    }

    p.sqrt()
}

pub fn outer<T: Num<T>>(v: &Vector<T>, w: &Vector<T>) -> Vector<Vector<T>> {
    let mut rows = Vector(Vec::with_capacity(v.len()));

    for ve in v.0.iter() {
        let mut row = Vec::with_capacity(w.len());
        for we in w.iter() {
            row.push(*ve * *we);
        }
        rows.0.push(Vector(row))
    }

    rows
}

pub fn dot<T: Num<T>>(v: &Vector<T>, w: &Vector<T>) -> T {
    assert_len(v.len(), w.len());

    let mut prod = T::zero();
    let dim = v.len();
    for i in 0..dim {
        prod = prod + v[i] * w[i];
    }

    prod
}

pub fn cross<T: Num<T>>(v: &Vector<T>, w: &Vector<T>) -> Vector<T> {
    assert_len(3, v.len());

    let x = v[1] * w[2] - v[2] * w[1];
    let y = v[2] * w[0] - v[0] * w[2];
    let z = v[0] * w[1] - v[1] * w[0];

    Vector(vec![x, y, z])
}

pub fn unit_vector<T: Num<T>>(v: &Vector<T>) -> Vector<T> {
    v / norm(v)
}
