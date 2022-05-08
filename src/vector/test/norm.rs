use num::Complex;

use crate::vector::*;

#[test]
fn f32() {
    let v: Vector<f32> = Vector(vec![1., 2., 3.]);

    let got = norm(&v);
    let expect = 3.7416575;
    assert_eq!(expect, got);
}

#[test]
fn f64() {
    let v: Vector<f64> = Vector(vec![1., 2., 3.]);

    let got = norm(&v);
    let expect = 3.7416573867739413;
    assert_eq!(expect, got);
}

#[test]
fn cmplx_f32() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f32>> = Vector(vec![x, y, z]);

    let got = norm(&v);
    let expect = Complex::new(0.015122108, -15.871289);
    assert_eq!(expect, got);
}

#[test]
fn cmplx_f64() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f64>> = Vector(vec![x, y, z]);

    let got = norm(&v);
    let expect = Complex::new(0.01512164470464187, -15.871289445540919);
    assert_eq!(expect, got);
}
