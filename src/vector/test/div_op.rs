use num::Complex;

use crate::vector::*;

#[test]
fn f32() {
    let v: Vector<f32> = Vector(vec![1., 3., 5.]);
    let s: f32 = 2.0;

    let got = v / s;
    let expect = Vector(vec![0.5, 1.5, 2.5]);
    assert_eq!(expect, got);
}

#[test]
fn f64() {
    let v: Vector<f64> = Vector(vec![1., 3., 5.]);
    let s: f64 = 2.0;

    let got = v / s;
    let expect = Vector(vec![0.5, 1.5, 2.5]);
    assert_eq!(expect, got);
}

#[test]
fn complx_f32() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 81.);

    let v: Vector<Complex<f32>> = Vector(vec![x, y, z]);
    let s: Complex<f32> = Complex::new(1., 3.1);

    let got = v / s;
    let expect = Vector(vec![x / s, y / s, z / s]);
    assert_eq!(expect, got)
}

#[test]
fn complx_f64() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 81.);

    let v: Vector<Complex<f64>> = Vector(vec![x, y, z]);
    let s: Complex<f64> = Complex::new(1., 3.1);

    let got = v / s;
    let expect = Vector(vec![x / s, y / s, z / s]);
    assert_eq!(expect, got)
}
