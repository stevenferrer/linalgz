use crate::vector::*;

#[test]
fn f32() {
    let v: Vector<f32> = Vector(vec![1., 2., 3.]);

    let got = unit_vector(&v);
    let expect = Vector(vec![0.26726124, 0.5345225, 0.8017837]);
    assert_eq!(expect, got);

    assert_eq!(1., norm(&got).ceil());
}

#[test]
fn f64() {
    let v: Vector<f64> = Vector(vec![1., 2., 3.]);

    let got = unit_vector(&v);
    let expect = Vector(vec![
        0.2672612419124244,
        0.5345224838248488,
        0.8017837257372732,
    ]);
    assert_eq!(expect, got);

    assert_eq!(1., norm(&got).ceil());
}
