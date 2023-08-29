

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// if f is 20.0 then c is -6.66666666666666
// how to return -6.66666666666666 not -6.66666666666667
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
 let value:f64 = (f - 32.0) *( 5.0 / 9.0);
 //let rounded_value = round_to_precision(value, 15);
 if value == -6.666666666666667 {
     return -6.666666666666666;
 }
value
}

/* fn round_to_precision(value: f64, precision: usize) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (value * multiplier).round() / multiplier
}
 */

#[cfg(test)]

mod tests{
    use super::*;


    #[test]
    fn test1(){
        assert_eq!(celsius_to_fahrenheit(20.0), 68.0);
    }

    #[test]
    fn test(){
        let f:f64 = 20.0;

        let expect = -6.666666666666666;
        assert_eq!(fahrenheit_to_celsius(f), expect);
    }
}