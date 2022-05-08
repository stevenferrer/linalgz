use num::Complex;

use crate::vector::*;

#[test]
fn scalar_f32() {
    let v: Vector<f32> = Vector(vec![1., 2., 3.]);
    let s = 2.;

    let got = v * s;
    let expect = Vector(vec![2., 4., 6.]);
    assert_eq!(expect, got);
}

#[test]
fn scalar_f64() {
    let v: Vector<f64> = Vector(vec![1., 2., 3.]);
    let s = 2.;

    let got = v * s;
    let expect = Vector(vec![2., 4., 6.]);
    assert_eq!(expect, got);
}

#[test]
fn scalar_cmplx_f32() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f32>> = Vector(vec![x, y, z]);
    let s = Complex::new(1.3, 8.3);

    let got = v * s;
    let expect = Vector(vec![x * s, y * s, z * s]);
    assert_eq!(expect, got);
}

#[test]
fn scalar_cmplx_f64() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f64>> = Vector(vec![x, y, z]);
    let s = Complex::new(1.3, 8.3);

    let got = v * s;
    let expect = Vector(vec![x * s, y * s, z * s]);
    assert_eq!(expect, got);
}

#[test]
fn element_wise_f32() {
    let v: Vector<f32> = Vector(vec![1., 3., 5.]);
    let w = Vector(vec![2., 2., 3.]);

    let got = v * w;
    let expect = Vector(vec![2., 6., 15.]);
    assert_eq!(expect, got);
}

#[test]
fn element_wise_f64() {
    let v: Vector<f64> = Vector(vec![1., 3., 5.]);
    let w = Vector(vec![2., 2., 3.]);

    let got = v * w;
    let expect = Vector(vec![2., 6., 15.]);
    assert_eq!(expect, got);
}

#[test]
fn element_wise_complx_f32() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f32>> = Vector(vec![x, y, z]);
    let w = Vector(vec![x, y, z]);

    let got = v * w;
    let expect = Vector(vec![x * x, y * y, z * z]);
    assert_eq!(expect, got);
}

#[test]
fn element_wise_complx_f64() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f64>> = Vector(vec![x, y, z]);
    let w = Vector(vec![x, y, z]);

    let got = v * w;
    let expect = Vector(vec![x * x, y * y, z * z]);
    assert_eq!(expect, got);
}
