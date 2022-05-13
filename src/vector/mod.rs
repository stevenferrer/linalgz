mod fns;
mod ops;

#[cfg(test)]
mod fns_test;
#[cfg(test)]
mod mod_test;
#[cfg(test)]
mod ops_test;

use std::fmt;
use std::slice::Iter;
use std::vec::Vec;

pub use crate::vector::fns::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T>(Vec<T>);

impl<T> Vector<T> {
    pub fn new(vec: Vec<T>) -> Vector<T> {
        Vector(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.0.iter()
    }

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
