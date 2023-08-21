/* copy
Instructions
Create the following functions. The objective is to know how ownership works with different types.

nbr_function returns a tuple:
with the original value.
the exponential function of the value.
and the natural logarithm of the absolute value.
str_function returns a tuple:
with the original value.
and the exponential function of each value as a string (see the example).
vec_function returns a tuple:
with the original value.
and the natural logarithm of each absolute value.
Expected functions
pub fn nbr_function(c: i32) -> (i32, f64, f64) {
}

pub fn str_function(a: String) -> (String, String) {
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
}
*/

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, (c as f64).exp(), (c as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut result = String::new();
    let mut result2 = String::new();
    let mut first = true;
    for i in a.split_whitespace() {
        if first {
            first = false;
        } else {
            result.push(' ');
            result2.push(' ');
        }
        let f = i.parse::<f64>().unwrap();
        result.push_str(&f.to_string());
        result2.push_str(&f.exp().to_string());
    }
    (a, result2)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut result = Vec::new();
    let mut result2 = Vec::new();
    for i in b {
        result.push(i);
        result2.push((i as f64).ln());
    }
    (result, result2)
}
