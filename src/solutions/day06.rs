//! # Solutions for Day 06 - Lanternfish
//!
//! For puzzle text, see https://adventofcode.com/2021/day/6
use crate::InputMode;

fn iterate_age_distr(age_distr: &mut [i64; 9], n: u64) {
    let mut num_procreating;

    for _ in 0..n {
        num_procreating = age_distr[0];
        age_distr.rotate_left(1);
        age_distr[8] = num_procreating;
        age_distr[6] += num_procreating;
    }
}

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let mut age_distr: [i64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for age in input.trim().split(",").map(|n| n.parse::<usize>().unwrap()) {
        age_distr[age] += 1;
    }

    iterate_age_distr(&mut age_distr, 80);
    return age_distr.iter().sum();
}

/// Implements the solution for part 2
pub fn solve_part2(input: &String, _input_mode: &InputMode) -> i64 {
    let mut age_distr: [i64; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for age in input.trim().split(",").map(|n| n.parse::<usize>().unwrap()) {
        age_distr[age] += 1;
    }

    iterate_age_distr(&mut age_distr, 256);
    return age_distr.iter().sum();
}
