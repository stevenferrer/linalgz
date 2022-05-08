use num_complex::Complex;

use crate::vector::*;

#[test]
fn f32() {
    let v: Vector<f32> = Vector(vec![1., 2., 3.]);
    let w: Vector<f32> = Vector(vec![1., 2., 6.]);

    let got = v + w;
    let expect = Vector(vec![2., 4., 9.]);
    assert_eq!(expect, got);
}

#[test]
fn f64() {
    let v: Vector<f64> = Vector(vec![1., 2., 3.]);
    let w: Vector<f64> = Vector(vec![1., 2., 6.]);

    let got = v + w;
    let expect = Vector(vec![2., 4., 9.]);
    assert_eq!(expect, got);
}

#[test]
fn cmplx_f32() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 81.);

    let v: Vector<Complex<f32>> = Vector(vec![x, y, z]);
    let w = Vector(vec![x, y, z]);

    let got = v + w;
    let expect = Vector(vec![x + x, y + y, z + z]);
    assert_eq!(expect, got)
}

#[test]
fn cmplx_f64() {
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 81.);

    let v: Vector<Complex<f64>> = Vector(vec![x, y, z]);
    let w = Vector(vec![x, y, z]);

    let got = v + w;
    let expect = Vector(vec![x + x, y + y, z + z]);
    assert_eq!(expect, got)
}
