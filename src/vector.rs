//! Vector type definition.

mod fns;
mod ops;

use std::fmt;
use std::slice::Iter;
use std::vec::Vec;

pub use crate::vector::fns::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T>(Vec<T>);

impl<T> Vector<T> {
    /// Initializes a new [Vector] from a [Vec].
    ///
    /// # Example
    /// ```
    /// use linalgz::*;
    ///
    /// let v = Vector::new(vec![1, 2, 3]);
    /// assert_eq!(v, vector![1, 2, 3])
    /// ```
    pub fn new(vec: Vec<T>) -> Vector<T> {
        Vector(vec)
    }

    /// Returns the [Vector] length.
    ///
    /// # Example
    /// ```
    /// use linalgz::*;
    ///
    /// let v = vector![1, 2, 3];
    /// assert_eq!(v.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the [Vector] is empty.
    ///
    /// # Example
    /// ```
    /// use linalgz::*;
    ///
    /// let v: Vector<i32> = vector![];
    /// assert!(v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an [Vector] iterator.
    ///
    /// # Example
    /// ```
    /// use linalgz::*;
    ///
    /// let v = vector![1, 2, 3];
    /// for e in v.iter() {
    ///     println!("{}", e)
    /// }
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }

    /// Appends a new element to the [Vector].
    ///
    /// # Example
    /// ```
    /// use linalgz::*;
    ///
    /// let mut v = vector![1, 2, 3];
    /// v.push(4);
    /// assert_eq!(v, vector![1, 2, 3, 4]);
    /// ```
    pub fn push(&mut self, val: T) {
        self.0.push(val)
    }
}

impl<T> fmt::Display for Vector<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = &self.0;

        write!(f, "[")?;
        for (cnt, e) in v.iter().enumerate() {
            if cnt != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", e)?;
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use crate::vector::*;
    use crate::*;

    #[test]
    fn test_fmt() {
        let v: Vector<f32> = vector![1.4, 1.23, 3.34];
        let w = vector![2.8, -1.1, 1.6];

        assert!(!v.is_empty());

        let k = outer(&v, &w);

        let expect = String::from(
            "[[3.9199998, -1.54, 2.24], [3.444, -1.353, 1.968], [9.351999, -3.674, 5.344]]",
        );
        assert_eq!(expect, format!("{}", k))
    }
}
