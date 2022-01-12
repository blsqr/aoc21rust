//! # Solutions for Day 07 - The Treachery of the Whales
//!
//! For puzzle text, see https://adventofcode.com/2021/day/7

use std::cmp;

use crate::InputMode;

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let mut positions = input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    positions.sort();

    let n = positions.len();
    println!("Have {} crab positions available ...", n);

    // Target position is simply the median
    let target_position = match n % 2 == 0 {
        true => ((positions[n / 2 - 1] + positions[n / 2]) as f64 / 2.).round() as i64,
        false => positions[n / 2],
    };

    // Compute fuel consumption
    positions
        .iter()
        .map(|pos| (pos - target_position).abs())
        .sum()
}

/// Computes the non-linear fuel consumption used in part 2
fn compute_fuel(pos: i64, target: i64) -> i64 {
    let mut f = 0;
    for n in 1..=(pos - target).abs() {
        f += n;
    }
    return f;
}

/// Implements the solution for part 2
pub fn solve_part2(input: &String, _input_mode: &InputMode) -> i64 {
    let mut positions = input
        .trim()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    positions.sort();

    let n = positions.len();
    println!("Have {} crab positions available ...", n);

    // Best guess for optimal target position: mean value
    let mean_pos: i64 = positions.iter().sum::<i64>() / n as i64;
    let mut minimum_fuel_consumption = i64::MAX;

    // Try out in a range around the mean value
    for target_pos in mean_pos - 10..=mean_pos + 10 {
        minimum_fuel_consumption = cmp::min(
            minimum_fuel_consumption,
            positions
                .iter()
                .map(|pos| compute_fuel(*pos, target_pos))
                .sum(),
        );
    }

    return minimum_fuel_consumption;
}
