/* matrix_ops
Instructions
In this exercise, you will define some basic matrix operations, Implement traits for Add and Sub

Remember that two matrices can only be added or subtracted if they have the same dimensions. Therefore, you must handle the possibility of failure by returning an Option<T>.

You will be reusing your Matrix and Scalar structures defined in the matrix and lalgebra_scalar exercises.

Expected Function
use crate::{Matrix, Scalar};
use std::ops::{ Add, Sub };

impl Add for Matrix {

}

impl Sub for Matrix {

}
Usage
Here is a program to test your function

use matrix_ops::*;

fn main() {
	let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
	let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix - matrix_2);

	let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
	let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
	println!("{:?}", matrix + matrix_2);
}
And its output

$ cargo run
Some(Matrix([[9, 2], [10, 2]]))
Some(Matrix([[-2, 2], [1, 4]]))
None
None
$ */

// Path: src/lib.rs

use lalgebra_scalar::Scalar;
//use crate::{Matrix, Scalar};
use std::ops::{ Add, Sub };

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn add(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0.iter().any(|row| row.len() != other.0[0].len()) {
            None // Return None if dimensions do not match
        } else {
            Some(Matrix(
                self.0
                    .iter()
                    .zip(other.0.iter())
                    .map(|(row1, row2)| {
                        row1.iter().zip(row2.iter()).map(|(a, b)| a.clone() + b.clone()).collect()
                    })
                    .collect(),
            ))
        }
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Clone,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, other: Matrix<T>) -> Option<Matrix<T>> {
        if self.0.len() != other.0.len() || self.0.iter().any(|row| row.len() != other.0[0].len()) {
            None // Return None if dimensions do not match
        } else {
            Some(Matrix(
                self.0
                    .iter()
                    .zip(other.0.iter())
                    .map(|(row1, row2)| {
                        row1.iter().zip(row2.iter()).map(|(a, b)| a.clone() - b.clone()).collect()
                    })
                    .collect(),
            ))
        }
    }
}

