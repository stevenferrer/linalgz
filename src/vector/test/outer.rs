use crate::vector::*;

#[test]
fn f32() {
    let v: Vector<f32> = Vector(vec![1., 2., 3.]);
    let w = Vector(vec![2., 4., 6.]);

    let got = outer(&v, &w);
    let expect = Vector(vec![
        Vector(vec![2., 4., 6.]),
        Vector(vec![4., 8., 12.]),
        Vector(vec![6., 12., 18.]),
    ]);

    assert_eq!(expect, got);
}

#[test]
fn f64() {
    let v: Vector<f64> = Vector(vec![1., 2., 3.]);
    let w = Vector(vec![2., 4., 6.]);

    let got = outer(&v, &w);
    let expect = Vector(vec![
        Vector(vec![2., 4., 6.]),
        Vector(vec![4., 8., 12.]),
        Vector(vec![6., 12., 18.]),
    ]);

    assert_eq!(expect, got);
}
