use crate::vector::*;

#[test]
fn index_ok() {
    let (x, y, z) = (1., 2., 3.);

    let v: Vector<f32> = Vector(vec![x, y, z]);

    let got = v[0];
    let expect = x;
    assert_eq!(expect, got);
}
