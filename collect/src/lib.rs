// todo: collect
/* 
Instructions
Implement the function bubble_sort, which receives a Vec<i32> and returns the same vector but in increasing order using the bubble sort algorithm.

Expected Function
pub fn bubble_sort(vec: &mut Vec<i32>) {
} */

pub fn bubble_sort(vec: &mut Vec<i32>) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 1..vec.len() {
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                swapped = true;
            }
        }
    }
}