//! # Solutions for Day 02 - Dive!

use crate::InputMode;

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let mut pos = [0, 0];
    let to_int = |v: &str| v.replace(" ", "").parse::<i64>().unwrap();

    for line in input.lines() {
        match line.split_at(line.find(" ").unwrap()) {
            ("forward", x) => pos[0] += to_int(x),
            ("up", y) => pos[1] -= to_int(y),
            ("down", y) => pos[1] += to_int(y),
            _ => panic!("Invalid input line: {:?}", line),
        }

        println!("Applied instruction:  {}\t  Now at: {:?}", line, pos);
    }

    println!("\nFinal position:  {:?}", pos);
    return pos[0] * pos[1];
}

/// Implements the solution for part 2
/// Same as part 1 but with a three-measurement sliding window
pub fn solve_part2(_input: &String, _input_mode: &InputMode) -> i64 {
    0
}
