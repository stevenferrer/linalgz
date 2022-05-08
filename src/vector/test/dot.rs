use crate::vector::*;

#[test]
fn f32() {
    let v: Vector<f32> = Vector(vec![1., 2., 3.]);
    let w = Vector(vec![1., 2., 3.]);

    let prod = dot(&v, &w);
    assert_eq!(14., prod)
}

#[test]
fn f64() {
    let v: Vector<f64> = Vector(vec![1., 2., 3.]);
    let w = Vector(vec![1., 2., 3.]);

    let prod = dot(&v, &w);
    assert_eq!(14., prod)
}
