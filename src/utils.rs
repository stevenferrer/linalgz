//! Misc helper utilities.

use crate::Vector;

/// Asserts that the given vectors have the same length and panics otherwise.
///
/// # Example
///
/// ```
/// use linalg::vector;
/// use linalg::utils::*;
///
/// let v1 = vector![1, 2, 3];
/// let v2 = vector![4, 5, 6];
/// assert_lens(&v1, &v2)
/// ```
pub fn assert_lens<T>(left: &Vector<T>, right: &Vector<T>) {
    if left.len() != right.len() {
        panic!(
            "unequal vector lengths: left: {}, right: {}",
            left.len(),
            right.len()
        );
    }
}

/// Asserts that the given vector have the given length and panics otherwise.
///
/// # Example
///
/// ```
/// use linalg::vector;
/// use linalg::utils::*;
///
/// let v = vector![1, 2, 3];
/// assert_len(&v, 3)
/// ```
pub fn assert_len<T>(v: &Vector<T>, len: usize) {
    if v.len() != len {
        panic!("expecting vector length {} but got {}", v.len(), len);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::vector;

    #[test]
    #[should_panic]
    fn unequal_lens() {
        let v1 = vector![1, 2, 3];
        let v2 = vector![1, 2, 3, 4];

        assert_lens(&v1, &v2)
    }

    #[test]
    #[should_panic]
    fn unexpected_len() {
        let v1 = vector![1, 2, 3];
        assert_len(&v1, 4);
    }
}
