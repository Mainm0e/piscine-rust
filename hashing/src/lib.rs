// todo: hashing
/* 
Instructions
Given a list of integers (Vec<i32>) write three functions.

-mean: that calculates the mean (the average value) of all the values in the list.

median: that calculates the median (for a sorted list, it is the value in the middle). If there is an even amount of numbers in the list, the middle pair must be determined, added together, and divided by two to find the median value.

mode that calculates the mode (the value that appears more often).

Expected Functions
pub fn mean(list: &Vec<i32>) -> f64 {
}

pub fn median(list: &Vec<i32>) -> i32 {
}

pub fn mode(list: &Vec<i32>) -> i32 {
} */

use std::collections::HashMap;

pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for i in list {
        sum += i;
    }
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut sorted = list.clone();
    sorted.sort();
    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid] + sorted[mid - 1]) / 2
    } else {
        sorted[mid]
    }
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for i in list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut mode = 0;
    for (k, v) in map {
        if v > max {
            max = v;
            mode = *k;
        }
    }
    mode
}
