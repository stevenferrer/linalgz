use crate::vector::*;

#[test]
fn f32() {
    let v: Vector<f32> = Vector(vec![1., 2., 3.]);
    let w = Vector(vec![4., 5., 6.]);

    let got = cross(&v, &w);
    let expect = Vector(vec![-3., 6., -3.]);
    assert_eq!(expect, got);
}

#[test]
fn f64() {
    let v: Vector<f64> = Vector(vec![1., 2., 3.]);
    let w = Vector(vec![4., 5., 6.]);

    let got = cross(&v, &w);
    let expect = Vector(vec![-3., 6., -3.]);
    assert_eq!(expect, got);
}
