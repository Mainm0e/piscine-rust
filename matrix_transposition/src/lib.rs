// todo :matrix_transposition
/* 
Instructions
Define a struct named Matrix as a tuple of 2 tuples. The nested tuple will contain 2 i32s.

Create a function named transpose that calculates the transposition of a 2x2 matrix.

pub fn transpose(m: Matrix) -> Matrix {
}
The transposition of a matrix, switches the columns to rows, and the rows to columns. For example:

( a b )   __ transposition __>   ( a c )
( c d )                          ( b d )
Matrix must implement Debug, PartialEq and Eq. You can use derive.

Remember that you are defining a library, so any element that can be called from an external crate must be made public. */

// Path: src\lib.rs
#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub(i32, i32), pub(i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    Matrix((m.0 .0, m.1 .0), (m.0 .1, m.1 .1))
}
