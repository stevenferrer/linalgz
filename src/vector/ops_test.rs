use num_complex::Complex;

use crate::vector::Vector;

#[test]
fn test_add() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 2., 3.]);
    let w: Vector<f32> = Vector::new(vec![1., 2., 6.]);

    let got = v + w;
    let expect = Vector::new(vec![2., 4., 9.]);
    assert_eq!(expect, got);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 2., 3.]);
    let w: Vector<f64> = Vector::new(vec![1., 2., 6.]);

    let got = v + w;
    let expect = Vector::new(vec![2., 4., 9.]);
    assert_eq!(expect, got);

    // complex
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 81.);

    let v: Vector<Complex<f32>> = Vector::new(vec![x, y, z]);
    let w = Vector::new(vec![x, y, z]);

    let got = v + w;
    let expect = Vector::new(vec![x + x, y + y, z + z]);
    assert_eq!(expect, got)
}

#[test]
fn test_sub() {
    // f32
    let v = Vector::new(vec![1., 2., 8.]);
    let w = Vector::new(vec![2., 3.5, 5.]);

    let got: Vector<f32> = v - w;
    let expect = Vector::new(vec![-1., -1.5, 3.]);
    assert_eq!(expect, got);

    // f64
    let v = Vector::new(vec![1., 2., 8.]);
    let w = Vector::new(vec![2., 3.5, 5.]);

    let got: Vector<f64> = v - w;
    let expect = Vector::new(vec![-1., -1.5, 3.]);
    assert_eq!(expect, got);

    // complex
    let x1 = Complex::new(1., 3.);
    let y1 = Complex::new(9., -1.);
    let z1 = Complex::new(0.32, 81.);

    let x2 = Complex::new(7., 3.);
    let y2 = Complex::new(0.9, 4.1);
    let z2 = Complex::new(0.32, 0.67);

    let v: Vector<Complex<f32>> = Vector::new(vec![x1, y1, z1]);
    let w = Vector::new(vec![x2, y2, z2]);

    let got = v - w;
    let expect = Vector::new(vec![x1 - x2, y1 - y2, z1 - z2]);
    assert_eq!(expect, got)
}

#[test]
fn test_scalar_mul() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 2., 3.]);
    let s = 2.;

    let got = v * s;
    let expect = Vector::new(vec![2., 4., 6.]);
    assert_eq!(expect, got);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 2., 3.]);
    let s = 2.;

    let got = v * s;
    let expect = Vector::new(vec![2., 4., 6.]);
    assert_eq!(expect, got);

    // complex
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f32>> = Vector::new(vec![x, y, z]);
    let s = Complex::new(1.3, 8.3);

    let got = v * s;
    let expect = Vector::new(vec![x * s, y * s, z * s]);
    assert_eq!(expect, got);
}

#[test]
fn test_element_wise() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 3., 5.]);
    let w = Vector::new(vec![2., 2., 3.]);

    let got = v * w;
    let expect = Vector::new(vec![2., 6., 15.]);
    assert_eq!(expect, got);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 3., 5.]);
    let w = Vector::new(vec![2., 2., 3.]);

    let got = v * w;
    let expect = Vector::new(vec![2., 6., 15.]);
    assert_eq!(expect, got);

    // complex
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 18.);

    let v: Vector<Complex<f32>> = Vector::new(vec![x, y, z]);
    let w = Vector::new(vec![x, y, z]);

    let got = v * w;
    let expect = Vector::new(vec![x * x, y * y, z * z]);
    assert_eq!(expect, got);
}

#[test]
fn test_div() {
    // f32
    let v: Vector<f32> = Vector::new(vec![1., 3., 5.]);
    let s: f32 = 2.0;

    let got = v / s;
    let expect = Vector::new(vec![0.5, 1.5, 2.5]);
    assert_eq!(expect, got);

    // f64
    let v: Vector<f64> = Vector::new(vec![1., 3., 5.]);
    let s: f64 = 2.0;

    let got = v / s;
    let expect = Vector::new(vec![0.5, 1.5, 2.5]);
    assert_eq!(expect, got);

    // complex
    let x = Complex::new(1., 3.);
    let y = Complex::new(9., -1.);
    let z = Complex::new(0.32, 81.);

    let v: Vector<Complex<f32>> = Vector::new(vec![x, y, z]);
    let s: Complex<f32> = Complex::new(1., 3.1);

    let got = v / s;
    let expect = Vector::new(vec![x / s, y / s, z / s]);
    assert_eq!(expect, got)
}

#[test]
fn test_index() {
    let (x, y, z) = (1., 2., 3.);

    let v: Vector<f32> = Vector::new(vec![x, y, z]);

    let got = v[0];
    let expect = x;
    assert_eq!(expect, got);
}
