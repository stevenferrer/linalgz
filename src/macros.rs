#[macro_export]
macro_rules! vector {
    () => (
        $crate::vector::Vector::new(std::vec::Vec::new())
    );
    ($($x:expr),*) => (
        $crate::vector::Vector::new(vec![$($x),*])
    );
    ($($x:expr,)*) => (
        $crate::vector::Vector::new(vec![$($x),*])
    )
}

#[macro_export]
macro_rules! complex {
    () => {
        num_complex::Complex::new(0, 0)
    };
    ($re:expr, $im:expr) => {
        num_complex::Complex::new($re, $im)
    };
}
