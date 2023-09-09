//todo:lalgebra_scalar
/* 
Instructions
Define a Scalar trait which implements the operations Add, Sub, Mul, Div and any other restrictions you may need. Use a trait inheritance (supertraits)

Another condition for a number to be a scalar is to have a zero (as the neutral element in the addition), and a one (as the neutral element in the multiplication). Therefore the Scalar trait will require these two functions (described below).

After finishing completing the declaration of the trait, implement the Scalar trait for u32, u64, i32, i64, f32 and f64.

Expected Function (The signature must be completed)
You need add the impl for each cases asked in the subject

pub trait Scalar: _ {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
} */


use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Clone + Sized {
    type Item; // Associated type Item
    fn zero() -> Self; // Method to return the "zero" of the type
    fn one() -> Self;  // Method to return the "one" of the type
    fn item(&self) -> Self::Item; // Method to return the associated item value
}

// Implementing Scalar for u32
impl Scalar for u32 {
    type Item = u32; // Associated type Item is u32

    fn zero() -> Self {
        0 // Zero for u32
    }

    fn one() -> Self {
        1 // One for u32
    }

    fn item(&self) -> Self::Item {
        *self // Return the value as the associated item
    }
}

// Implementing Scalar for u64 (similarly for other types)
impl Scalar for u64 {
    type Item = u64;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for i32
impl Scalar for i32 {
    type Item = i32;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for i64 (similarly for other types)
impl Scalar for i64 {
    type Item = i64;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for f32
impl Scalar for f32 {
    type Item = f32;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for f64 (similarly for other types)
impl Scalar for f64 {
    type Item = f64;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn item(&self) -> Self::Item {
        *self
    }
}




 /* 
use std::ops::{Add, Sub, Mul, Div};

pub trait Scalar: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Sized {
    type Item; // Associated type Item
    fn zero() -> Self; // Method to return the "zero" of the type
    fn one() -> Self;  // Method to return the "one" of the type
    fn item(&self) -> Self::Item; // Method to return the associated item value
}

// Implementing Scalar for u32
impl Scalar for u32 {
    type Item = u32; // Associated type Item is u32

    fn zero() -> Self {
        0 // Zero for u32
    }

    fn one() -> Self {
        1 // One for u32
    }

    fn item(&self) -> Self::Item {
        *self // Return the value as the associated item
    }
}

// Implementing Scalar for u64 (similarly for other types)
impl Scalar for u64 {
    type Item = u64;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for i32
impl Scalar for i32 {
    type Item = i32;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for i64 (similarly for other types)
impl Scalar for i64 {
    type Item = i64;

    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for f32
impl Scalar for f32 {
    type Item = f32;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn item(&self) -> Self::Item {
        *self
    }
}

// Implementing Scalar for f64 (similarly for other types)
impl Scalar for f64 {
    type Item = f64;

    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn item(&self) -> Self::Item {
        *self
    }
}
 */