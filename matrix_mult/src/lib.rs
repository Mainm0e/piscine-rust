/* matrix_mult
Instructions
Implement the methods:

number_of_cols: which returns the number of columns in the matrix.
number_of_rows: which returns the number of rows in the matrix.
row: which returns the nth row in the matrix.
col: which returns the nth column in the matrix.
Define the matrix multiplication by implementing the std::ops::Mul for the type matrix

Expected Functions
impl Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
	}

	pub fn number_of_rows(&self) -> usize {
	}

	pub fn row(&self, n: usize) -> Vec<T> {
	}

	pub fn col(&self, n: usize) -> Vec<T> {
	}
}

impl Mul for Matrix<T> {
}
Usage
Here is a program to test your function.

use matrix_mult::*;

fn main() {
	let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
	println!("{:?}", matrix.col(0));
	println!("{:?}", matrix.row(1));

	let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
	let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
	let mult = matrix_1.clone() * matrix_2.clone();
	println!("{:?}", mult);
	println!("{:?}", matrix_1.number_of_cols());
	println!("{:?}", matrix_2.number_of_rows());
}
And its output

$ cargo run
[3, 8]
[8, 0]
Some(Matrix([[1, 0], [0, 0]]))
2
2
$ */




use std::ops::Mul;
use std::ops::Add;

#[derive(Debug,Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T :Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        if n < self.number_of_rows() {
            self.0[n].clone()
        } else {
            Vec::new()
        }
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        if n < self.number_of_cols() {
            let mut column = Vec::with_capacity(self.number_of_rows());
            for row in &self.0 {
                column.push(row[n].clone());
            }
            column
        } else {
            Vec::new()
        }
    }
}

impl<T: Mul<Output = T> + Clone + Default + Add<Output = T>> Mul for Matrix<T> {
    type Output = Self;


    fn mul(self, rhs: Self) -> Self {
        let mut result = Vec::with_capacity(self.number_of_rows());
        for row in &self.0 {
            let mut new_row = Vec::with_capacity(rhs.number_of_cols());
            for col in 0..rhs.number_of_cols() {
                let mut sum = T::default();
                for (i, val) in row.iter().enumerate() {
                    sum = sum + val.clone() * rhs.0[i][col].clone();
                }
                new_row.push(sum);
            }
            result.push(new_row);
        }
        Matrix(result)
    }
}