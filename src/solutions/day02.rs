//! # Solutions for Day 02 - Dive!
//!
//! For puzzle text, see https://adventofcode.com/2021/day/2

use crate::InputMode;

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let mut pos = [0, 0]; // x, y
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
pub fn solve_part2(input: &String, _input_mode: &InputMode) -> i64 {
    #[derive(Debug)]
    struct State {
        x: i64,
        y: i64,
        aim: i64,
    }

    let mut state = State { x: 0, y: 0, aim: 0 };
    let to_int = |v: &str| v.replace(" ", "").parse::<i64>().unwrap();

    for line in input.lines() {
        match line.split_at(line.find(" ").unwrap()) {
            ("forward", delta_x) => {
                state.x += to_int(delta_x);
                state.y += to_int(delta_x) * state.aim;
            }
            ("up", delta_aim) => {
                state.aim -= to_int(delta_aim);
            }
            ("down", delta_aim) => {
                state.aim += to_int(delta_aim);
            }
            _ => panic!("Invalid input line: {:?}", line),
        }

        println!("Applied instruction:  {}\t  ==>  {:?}", line, state);
    }

    println!("\nFinal position:  ({}, {})", state.x, state.y);
    return state.x * state.y;
}
