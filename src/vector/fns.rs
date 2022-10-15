use crate::traits::Num;
use crate::utils::{assert_len, assert_lens};
use crate::vector::Vector;

/// Calculates the norm for a given [Vector].
///
/// # Example
/// ```
/// use linalgz::*;
///
/// let v: Vector<f32> = vector![1., 2., 3.];
/// let got = norm(&v);
/// let expect = 3.7416575;
/// assert_eq!(expect, got);
/// ```
pub fn norm<T>(v: &Vector<T>) -> T
where
    T: Num<T>,
{
    let mut p = T::zero();
    for e in v.iter() {
        p = p + *e * *e;
    }

    p.sqrt()
}

/// Calculates teh outer product for the given [Vector]s.
///
/// # Example
/// ```
/// use linalgz::*;
///
/// let v = vector![1., 2., 3.];
/// let w = vector![2., 4., 6.];
///
/// let expect = vector![
///     vector![2., 4., 6.],
///     vector![4., 8., 12.],
///     vector![6., 12., 18.],
/// ];
/// assert_eq!(expect, outer(&v, &w));
/// ```
pub fn outer<T>(v: &Vector<T>, w: &Vector<T>) -> Vector<Vector<T>>
where
    T: Num<T>,
{
    let mut rows = Vector(Vec::with_capacity(v.len()));

    for ve in v.iter() {
        let mut row = Vec::with_capacity(w.len());
        for we in w.iter() {
            row.push(*ve * *we);
        }

        rows.push(Vector(row));
    }

    rows
}

/// Calculates the dot product for the given [Vector]s.
///
/// # Example
/// ```
/// use linalgz::*;
///
/// let v = vector![1., 2., 3.];
/// let w = vector![1., 2., 3.];
///
/// assert_eq!(14., dot(&v, &w));
/// ```
pub fn dot<T>(v: &Vector<T>, w: &Vector<T>) -> T
where
    T: Num<T>,
{
    assert_lens(v, w);

    let mut prod = T::zero();
    let dim = v.len();
    for i in 0..dim {
        prod = prod + v[i] * w[i];
    }

    prod
}

/// Calculates the cross product for the given [Vector].
///
/// # Example
/// ```
/// use linalgz::*;
///
/// let v = vector![1., 2., 3.];
/// let w = vector![4., 5., 6.];
///
/// let expect = vector![-3., 6., -3.];
/// assert_eq!(expect, cross(&v, &w));
/// ```
pub fn cross<T>(v: &Vector<T>, w: &Vector<T>) -> Vector<T>
where
    T: Num<T>,
{
    assert_len(v, 3);

    let x = v[1] * w[2] - v[2] * w[1];
    let y = v[2] * w[0] - v[0] * w[2];
    let z = v[0] * w[1] - v[1] * w[0];

    Vector(vec![x, y, z])
}

/// Calculates a unit vector for a given [Vector].
///
/// # Example
/// ```
/// use linalgz::*;
///
/// let v: Vector<f32> = vector![1., 2., 3.];
///
/// let got = unit_vector(&v);
/// let expect = vector![0.26726124, 0.5345225, 0.8017837];
///
/// assert_eq!(expect, got);
/// assert_eq!(1., norm(&got).ceil());
/// ```
pub fn unit_vector<T>(v: &Vector<T>) -> Vector<T>
where
    T: Num<T>,
{
    v / norm(v)
}

#[cfg(test)]
mod test {
    use num_complex::Complex;

    use crate::vector::*;
    use crate::*;

    #[test]
    fn test_norm() {
        // f32
        let v: Vector<f32> = vector![1., 2., 3.];

        let got = norm(&v);
        let expect = 3.7416575;
        assert_eq!(expect, got);

        // f64
        let v: Vector<f64> = vector![1., 2., 3.];

        let got = norm(&v);
        let expect = 3.7416573867739413;
        assert_eq!(expect, got);

        // complex f32
        let x = complex!(1., 3.);
        let y = complex!(9., -1.);
        let z = complex!(0.32, 18.);

        let v: Vector<Complex<f32>> = vector![x, y, z];

        let got = norm(&v);
        let expect = complex!(0.015122108, -15.871289);
        assert_eq!(expect, got);

        // complex f64
        let x = complex!(1., 3.);
        let y = complex!(9., -1.);
        let z = complex!(0.32, 18.);

        let v: Vector<Complex<f64>> = vector![x, y, z];

        let got = norm(&v);
        let expect = complex!(0.01512164470464187, -15.871289445540919);
        assert_eq!(expect, got);
    }

    #[test]
    fn test_outer() {
        // f32
        let v: Vector<f32> = vector![1., 2., 3.];
        let w = vector![2., 4., 6.];

        let got = outer(&v, &w);
        let expect = vector![
            vector![2., 4., 6.],
            vector![4., 8., 12.],
            vector![6., 12., 18.],
        ];
        assert_eq!(expect, got);

        // f64
        let v: Vector<f64> = vector![1., 2., 3.];
        let w = vector![2., 4., 6.];

        let got = outer(&v, &w);
        let expect = vector![
            vector![2., 4., 6.],
            vector![4., 8., 12.],
            vector![6., 12., 18.],
        ];
        assert_eq!(expect, got);
    }

    #[test]
    fn test_dot() {
        // f32
        let v: Vector<f32> = vector![1., 2., 3.];
        let w = vector![1., 2., 3.];

        let prod = dot(&v, &w);
        assert_eq!(14., prod);

        // f64
        let v: Vector<f64> = vector![1., 2., 3.];
        let w = vector![1., 2., 3.];

        let prod = dot(&v, &w);
        assert_eq!(14., prod);
    }

    #[test]
    fn test_cross() {
        // f32
        let v: Vector<f32> = vector![1., 2., 3.];
        let w = vector![4., 5., 6.];

        let got = cross(&v, &w);
        let expect = vector![-3., 6., -3.];
        assert_eq!(expect, got);

        // f64
        let v: Vector<f64> = vector![1., 2., 3.];
        let w = vector![4., 5., 6.];

        let got = cross(&v, &w);
        let expect = vector![-3., 6., -3.];
        assert_eq!(expect, got);
    }

    #[test]
    fn test_unit_vector() {
        // f32
        let v: Vector<f32> = vector![1., 2., 3.];

        let got = unit_vector(&v);
        let expect = vector![0.26726124, 0.5345225, 0.8017837];
        assert_eq!(expect, got);
        assert_eq!(1., norm(&got).ceil());

        // f64
        let v: Vector<f64> = vector![1., 2., 3.];

        let got = unit_vector(&v);
        let expect = vector![0.2672612419124244, 0.5345224838248488, 0.8017837257372732,];
        assert_eq!(expect, got);

        assert_eq!(1., norm(&got).ceil());
    }
}
