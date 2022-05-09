use linalg::vector::Vector;

fn main() {
    let a: Vector<f32> = Vector::new(vec![1.1, 4.3, 6.2]);
    let b = Vector::new(vec![0.8, 0.12, 0.4]);
    let s = 2.;

    println!("a: {}", a);
    println!("b: {}", b);

    // Addition
    let sum = &a + &b;
    println!("sum: {}", sum);

    // Subtraction
    let diff = &a - &b;
    println!("difference: {}", diff);

    // Vector-scalar multiplication
    let prod = &a * s;
    println!("vector-scalar product: {}", prod);

    let prod = &a * &b;
    println!("vector-vector product: {}", prod);

    // Vector-scalar division
    let quo = &a / s;
    println!("quotient: {}", quo);
}
