//use std::time::Instant;
//use crate::bubble::*;
use bubble_sort::bubble_sort;

fn main() {
    let mut numbers = [420, 69, 42, 13, 99, 710, 41, 123, 256, 33, 54, 24, 12, 22, 60, 1, 66, 84, 32, 78, 84, 36, 26, 74, 90, 11, 31, 81, 51, 0, 34, 44, 40, 37, 45, 58, 83, 55, 76, 17, 6];
    let total = numbers.len();
    println!("Sorting {total} numbers...");
    println!("Before: {:?}", numbers);
    bubble_sort(&mut numbers);
    println!("After: {:?}", numbers);
}
