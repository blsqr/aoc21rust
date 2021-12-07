//! # Solutions for Day 01 - Sonar Sweep

/// Implements the solution for part 1
pub fn solve_part1(input: &String) -> i64 {
    // Convert to vector of integers
    let depths = input.lines().collect::<Vec<_>>();
    depths.len() as i64
}
