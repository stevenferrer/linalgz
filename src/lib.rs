use num::traits::Float;

pub mod vector;

pub trait Num: Float {}
impl<T> Num for T where T: Float {}

#[derive(PartialEq, Debug, Clone)]
pub struct Vector<T>(Vec<T>);
