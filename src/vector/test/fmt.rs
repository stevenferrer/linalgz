use crate::vector::*;

#[test]
fn fmt() {
    let v: Vector<f32> = Vector(vec![1.4, 1.23, 3.34]);
    let w = Vector(vec![2.8, -1.1, 1.6]);

    let k = outer(&v, &w);

    let expect = String::from(
        "[[3.9199998, -1.54, 2.24], [3.444, -1.353, 1.968], [9.351999, -3.674, 5.344]]",
    );
    assert_eq!(expect, format!("{}", k))
}
