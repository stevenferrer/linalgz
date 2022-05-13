use linalg::vector::*;

fn main() {
    let a: Vector<f32> = Vector::new(vec![1., 4., 6.]);
    let b = Vector::new(vec![0.5, 0.7, 0.4]);
    let s = 2.;

    println!("a: {}", a);
    println!("b: {}", b);

    // Addition
    println!("add.: {}", &a + &b);

    // Subtraction
    println!("sub.: {}", &a - &b);

    // Vector-scalar multiplication
    println!("scalar mul.: {}", &a * s);

    println!("vector mul.: {}", &a * &b);

    // Vector-scalar division
    println!("scalar div.: {}", &a / s);

    // Vector magnitude
    println!("magnitude: {}", norm(&a));

    // Outer product
    println!("outer prod.: {}", outer(&a, &b));

    // Dot product
    println!("dot prod.: {}", dot(&a, &b));

    // Cross product
    println!("cross prod.: {}", cross(&a, &b));

    // Unit vector
    println!("unit vector: {}", unit_vector(&a));
}
