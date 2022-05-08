use num_complex::Complex;

use crate::vector::*;

#[test]
fn f32() {
    let v = Vector(vec![1., 2., 8.]);
    let w = Vector(vec![2., 3.5, 5.]);

    let got: Vector<f32> = v - w;
    let expect = Vector(vec![-1., -1.5, 3.]);
    assert_eq!(expect, got);
}

#[test]
fn f64() {
    let v = Vector(vec![1., 2., 8.]);
    let w = Vector(vec![2., 3.5, 5.]);

    let got: Vector<f64> = v - w;
    let expect = Vector(vec![-1., -1.5, 3.]);
    assert_eq!(expect, got);
}

#[test]
fn cmplx_f32() {
    let x1 = Complex::new(1., 3.);
    let y1 = Complex::new(9., -1.);
    let z1 = Complex::new(0.32, 81.);

    let x2 = Complex::new(7., 3.);
    let y2 = Complex::new(0.9, 4.1);
    let z2 = Complex::new(0.32, 0.67);

    let v: Vector<Complex<f32>> = Vector(vec![x1, y1, z1]);
    let w = Vector(vec![x2, y2, z2]);

    let got = v - w;
    let expect = Vector(vec![x1 - x2, y1 - y2, z1 - z2]);
    assert_eq!(expect, got)
}

#[test]
fn cmplx_f64() {
    let x1 = Complex::new(1., 3.);
    let y1 = Complex::new(9., -1.);
    let z1 = Complex::new(0.32, 81.);

    let x2 = Complex::new(7., 3.);
    let y2 = Complex::new(0.9, 4.1);
    let z2 = Complex::new(0.32, 0.67);

    let v: Vector<Complex<f64>> = Vector(vec![x1, y1, z1]);
    let w = Vector(vec![x2, y2, z2]);

    let got = v - w;
    let expect = Vector(vec![x1 - x2, y1 - y2, z1 - z2]);
    assert_eq!(expect, got)
}
