// todo: lalgebra_vector
/* 
Instructions
A vector in linear algebra is defined as "anything that can be added, and that can be multiplied by a scalar".

Define the associated function dot, that calculates the dot product between two vectors. If the vectors are of different lengths, return None.

Note: Vector must implement Debug, Clone, Eq and PartialEq.

Expected Functions and Structure
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;

impl Add for Vector<T> {
}

impl Vector<T> {
	pub fn new() -> Self {
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
	}
} */
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;
use std::fmt::Debug;
use std::cmp::PartialEq;
use std::clone::Clone;

pub trait Scalar: Add<Output = T> + Debug + Clone + PartialEq + Eq + Copy + Default {}

impl<T> Scalar for T where T: Add<Output = T> + Debug + Clone + PartialEq + Eq + Copy + Default {}

impl<T: Scalar> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Vector<T>) -> Self::Output {
        let mut result = Vec::new();
        for (a, b) in self.0.into_iter().zip(other.0.into_iter()) {
            result.push(a + b);
        }
        Vector(result)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Vector<T>) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = T::default();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            result = result + *a * *b;
        }
        Some(result)
    }
}

fn main() {
    // You can test your code here.
}
