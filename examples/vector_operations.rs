use linalg::vector::*;

fn main() {
    let a: Vector<f32> = Vector::new(vec![1.1, 4.3, 6.2]);
    let b = Vector::new(vec![0.8, 0.12, 0.4]);
    let s = 2.;

    println!("a: {}", a);
    println!("b: {}", b);

    // Addition
    println!("sum: {}", &a + &b);

    // Subtraction
    println!("difference: {}", &a - &b);

    // Vector-scalar multiplication
    println!("vector-scalar product: {}", &a * s);

    println!("vector-vector product: {}", &a * &b);

    // Vector-scalar division
    println!("quotient: {}", &a / s);

    // Vector magnitude
    println!("magnitude: {}", norm(&a));

    // Outer product
    println!("outer: {}", outer(&a, &b));

    // Dot product
    println!("dot: {}", dot(&a, &b));

    // Cross product
    println!("cross: {}", cross(&a, &b));

    // Unit vector
    println!("unit vector: {}", unit_vector(&a));
}
