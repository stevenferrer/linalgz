use num_complex::Complex;

use crate::vector::Vector;
use crate::vector::*;

#[test]
fn test_norm() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 2., 3.]);

    let got = norm(&v);
    let expect = 3.7416575;
    assert_eq!(expect, got);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 2., 3.]);

    let got = norm(&v);
    let expect = 3.7416573867739413;
    assert_eq!(expect, got);

    // complx
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f32>> = Vector::new(vec![x, y, z]);

    let got = norm(&v);
    let expect = Complex::new(0.015122108, -15.871289);
    assert_eq!(expect, got);
}

#[test]
fn test_outer() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 2., 3.]);
    let w = Vector::new(vec![2., 4., 6.]);

    let got = outer(&v, &w);
    let expect = Vector::new(vec![
        Vector::new(vec![2., 4., 6.]),
        Vector::new(vec![4., 8., 12.]),
        Vector::new(vec![6., 12., 18.]),
    ]);
    assert_eq!(expect, got);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 2., 3.]);
    let w = Vector::new(vec![2., 4., 6.]);

    let got = outer(&v, &w);
    let expect = Vector::new(vec![
        Vector::new(vec![2., 4., 6.]),
        Vector::new(vec![4., 8., 12.]),
        Vector::new(vec![6., 12., 18.]),
    ]);
    assert_eq!(expect, got);
}

#[test]
fn test_dot() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 2., 3.]);
    let w = Vector::new(vec![1., 2., 3.]);

    let prod = dot(&v, &w);
    assert_eq!(14., prod);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 2., 3.]);
    let w = Vector::new(vec![1., 2., 3.]);

    let prod = dot(&v, &w);
    assert_eq!(14., prod);
}

#[test]
fn test_cross() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 2., 3.]);
    let w = Vector::new(vec![4., 5., 6.]);

    let got = cross(&v, &w);
    let expect = Vector::new(vec![-3., 6., -3.]);
    assert_eq!(expect, got);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 2., 3.]);
    let w = Vector::new(vec![4., 5., 6.]);

    let got = cross(&v, &w);
    let expect = Vector::new(vec![-3., 6., -3.]);
    assert_eq!(expect, got);
}

#[test]
fn test_unit_vector() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 2., 3.]);

    let got = unit_vector(&v);
    let expect = Vector::new(vec![0.26726124, 0.5345225, 0.8017837]);
    assert_eq!(expect, got);
    assert_eq!(1., norm(&got).ceil());

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 2., 3.]);

    let got = unit_vector(&v);
    let expect = Vector::new(vec![
        0.2672612419124244,
        0.5345224838248488,
        0.8017837257372732,
    ]);
    assert_eq!(expect, got);

    assert_eq!(1., norm(&got).ceil());
}