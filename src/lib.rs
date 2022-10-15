//! # linalgz
//!
//! `linalgz` is a simple linear algebra library for Rust.

mod traits;

pub mod macros;
pub mod utils;
pub mod vector;

pub use crate::vector::Vector;
pub use crate::vector::{cross, dot, norm, outer, unit_vector};
