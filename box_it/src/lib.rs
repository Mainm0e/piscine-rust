// todo: box_it
/* 
Instructions
Create the following functions:

transform_and_save_on_heap: which accepts a string of numbers separated by spaces. If a number has a 'k' as a suffix it should be multiplied by 1000. 
The function transforms those numbers into a vector of u32, and saves them in the heap using Box.

take_value_ownership: which accepts the return value from transform_and_save_on_heap, unboxes the value, and returns it.

Expected Functions
pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {

}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {

} */




pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut v: Vec<u32> = Vec::new();  // Move this line outside of the loop
    for i in s.split_whitespace() {
        let mut num = i.to_string();
        if num.contains("k") {
            num.pop();
            if let Ok(parsed_num) = num.parse::<f32>(){
                v.push((parsed_num * 1000.0) as u32);
            }
        } else {
            let num: u32 = num.parse().unwrap();
            v.push(num);
        }
    }
    Box::new(v)
}


// assert_eq!(a, vec![5500, 8900, 32]);
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for i in a.iter() {
        v.push(*i);
    }
    v
}