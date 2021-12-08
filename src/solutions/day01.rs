//! # Solutions for Day 01 - Sonar Sweep
//!
//! For puzzle text, see https://adventofcode.com/2021/day/1

use crate::InputMode;

/// Implements the solution for part 1
///
/// Simply counts the number of times the depth increases in the input data
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let mut previous_depth = -1;
    let mut num_increasing = -1; // starting at -1 to account for first value

    // Iterate over lines, mapped to integer values
    // NOTE Avoiding to collect into a vector to allow lazy evaluation
    for depth in input.lines().map(|x| x.parse::<i64>().unwrap()) {
        if previous_depth < depth {
            num_increasing += 1;
        }
        previous_depth = depth;
    }
    return num_increasing;
}

/// Implements the solution for part 2
///
/// Same as part 1 but with a three-measurement sliding window
pub fn solve_part2(input: &String, input_mode: &InputMode) -> i64 {
    let mut sum_of_depths;
    let mut previous_sum_of_depths = -1;
    let mut num_increasing = -1; // starting at -1 to account for first value

    // Prepare iterators in a way that they are shifted relatively to each other
    let map_to_int = |x: &str| x.parse::<i64>().unwrap();

    let iter1 = input.lines().map(map_to_int);

    let mut iter2 = input.lines().map(map_to_int);
    iter2.next();

    let mut iter3 = input.lines().map(map_to_int);
    iter3.next();
    iter3.next();

    // Iterate over zipped iterators, creating a three-measurement window
    for ((d1, d2), d3) in iter1.zip(iter2).zip(iter3) {
        sum_of_depths = d1 + d2 + d3;

        if matches!(input_mode, InputMode::Test) {
            println!("{:?}  ->  {}", (d1, d2, d3), sum_of_depths);
        }

        if previous_sum_of_depths < sum_of_depths {
            num_increasing += 1;
        }
        previous_sum_of_depths = sum_of_depths;
    }
    return num_increasing;
}
