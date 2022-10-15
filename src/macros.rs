//! Macro definitions.

/// Similar to [vec!] macro but initializes [crate::Vector] instead.
///
/// # Example
///
/// ```
/// use linalgz::*;
///
/// let v = vector![1, 2, 3];
/// assert_eq!(v, Vector::new(vec![1, 2, 3]));
/// ```
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

/// Helper macro for initializing [num_complex::Complex].
///
/// # Example
/// ```
/// use linalgz::*;
/// use num_complex::Complex;
///
/// let x = complex!(7., 3.);
/// assert_eq!(x, Complex::new(7., 3.));
#[macro_export]
macro_rules! complex {
    () => {
        num_complex::Complex::new(0, 0)
    };
    ($re:expr, $im:expr) => {
        num_complex::Complex::new($re, $im)
    };
}
