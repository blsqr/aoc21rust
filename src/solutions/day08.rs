//! # Solutions for Day 08 - Seven Segment Search
//!
//! For puzzle text, see https://adventofcode.com/2021/day/8

use crate::InputMode;

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    // Parse data into a vector of vectors of output segment strings
    let all_outputs = input
        .trim()
        .lines()
        .map(|line| {
            line.split("|").collect::<Vec<_>>()[1]
                .trim()
                .split(" ")
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Count number of appearances of "easy digits" of length 2, 3, 4, 7
    let easy_digit_lengths = vec![2, 3, 4, 7];
    let mut num_easy_digits = 0;
    for outputs in all_outputs {
        for output in outputs {
            if easy_digit_lengths.contains(&output.len()) {
                num_easy_digits += 1;
            }
        }
    }

    return num_easy_digits;
}

/// Implements the solution for part 2
pub fn solve_part2(input: &String, _input_mode: &InputMode) -> i64 {
    0
}
