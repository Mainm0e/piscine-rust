/*
## copy

### Instructions

Your objective is to fix the program so that all functions work.

- `nbr_function`, returns a tuple with the original value, the exponential and logarithm of that value
- `str_function`, returns a tuple with the original value and the exponential of that value as a string
- `vec_function`, returns a tuple with the original value and the logarithm of each value

The objective is to now how ownership works with different types.

### Example

```rust
fn main() {
    let a = String::from("1 2 4 5 6");
    let b = vec![1, 2, 4, 5];
    let c: u32 = 0;

    println!("{:?}", nbr_function(c));
    // output: (12, 162754.79141900392, 2.4849066497880004)
    println!("{:?}", vec_function(b));
    // output: ([1, 2, 4], [0.0, 0.6931471805599453, 1.3862943611198906])
    println!("{:?}", str_function(a));
    // output: ("1 2 4", "2.718281828459045 7.38905609893065 54.598150033144236")
}
```

### Notions

- https://doc.rust-lang.org/rust-by-example/scope/move.html
*/

use copy::*;

#[allow(dead_code)]
fn main() {
    let a: i32 = 0;
    let b = String::from("1 2 4 5 6");
    let c = vec![1, 2, 4, 5];

    println!("{:?}", nbr_function(a));
    //(0, 1.0, -inf)
    println!("{:?}", str_function(b));
    //("1 2 4 5 6", "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351")
    println!("{:?}", vec_function(c));
    //([1, 2, 4, 5], [0.0, 0.6931471805599453, 1.3862943611198906, 1.6094379124341003])
}

#[test]
fn ownership_nbr_test() {
    assert_eq!(
        nbr_function(12),
        (12, 162754.79141900392, 2.4849066497880004)
    );
    assert_eq!(nbr_function(1), (1, 2.718281828459045, 0.0));
    assert_eq!(nbr_function(0), (0, 1.0, std::f64::NEG_INFINITY));
}

#[test]
fn negative_nbr_test() {
    assert_eq!(
        nbr_function(-12),
        (-12, 0.00000614421235332821, 2.4849066497880004)
    );
    assert_eq!(nbr_function(-1), (-1, 0.36787944117144233, 0.0));
    assert_eq!(nbr_function(0), (0, 1.0, std::f64::NEG_INFINITY));
}

#[test]
fn ownership_vec_test() {
    assert_eq!(
        vec_function(vec![1, 2, 4]),
        (
            vec![1, 2, 4],
            vec![0.0, 0.6931471805599453, 1.3862943611198906]
        )
    );

    assert_eq!(
        vec_function(vec![0, 1]),
        (vec![0, 1], vec![std::f64::NEG_INFINITY, 0.0])
    );

    assert_eq!(
        vec_function(vec![1, 2, 4, 5]),
        (
            vec![1, 2, 4, 5],
            vec![
                0.0,
                0.6931471805599453,
                1.3862943611198906,
                1.6094379124341003
            ]
        )
    );
}

#[test]
fn ownership_str_test() {
    assert_eq!(
        str_function(String::from("1 2 4")),
        (
            "1 2 4".to_string(),
            "2.718281828459045 7.38905609893065 54.598150033144236".to_string()
        )
    );
    assert_eq!(
        str_function(String::from("1 0")),
        (("1 0".to_string(), "2.718281828459045 1".to_string()))
    );
    assert_eq!(str_function(
			String::from("1 2 4 5 6")),
			(("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string())));
}