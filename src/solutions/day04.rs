//! # Solutions for Day 04 - Giant Squid
//!
//! For puzzle text, see https://adventofcode.com/2021/day/4

use std::collections::HashMap;
use itertools::iproduct;

#[allow(unused_imports)]
use ndarray::prelude::*;

use crate::InputMode;

type Board = ndarray::Array2<i64>;
type Winners = HashMap<usize, i64>;  // maps winning position to score

/// Parses string input and returns the numbers to draw and the bingo boards
fn read_numbers_and_boards(input: &String) -> (Vec<i64>, Vec<Board>) {
    let mut lines_iter = input.lines();
    let numbers = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut boards = Vec::<Board>::new();
    let mut current_board = Board::zeros((6, 6));
    let mut row = 0;
    for line in lines_iter {
        if line.len() == 0 {
            // Nothing to do
            continue;
        }

        // Set the corresponding row in the current board
        for (col, val) in line.split_whitespace().map(|n| n.parse::<i64>().unwrap()).enumerate() {
            current_board.row_mut(row)[col] = val;
        }

        // println!("line: {:?}", line);
        // println!("row: {}", row);
        // println!("current_board:\n{:?}", current_board);

        // Only need to continue if this was the 5th row added
        row += 1;
        if row < 5 {
            continue;
        }
        // Board finished
        // Compute sum of all numbers and write to last entry
        current_board[[5,5]] = current_board.sum();
        // println!("Board {} finished:\n{:?}", boards.len()+1, current_board);

        // Now track it and reset variables for next board
        boards.push(current_board);
        current_board = Board::zeros((6,6));
        row = 0;
    }

    return (numbers, boards);
}

/// Goes through the boards, marks matching numbers, and updates counters
///
/// The counters in the last row and column make it easier to look for winning
/// rows or columns. The counter in the bottom right-hand entry keeps track of
/// the sum of unmarked numbers
fn mark_boards(number: i64, boards: &mut Vec<Board>) {
    for (n, board) in boards.iter_mut().enumerate() {
        // Seems like rust ndarray has no easy way of finding the multi-index of
        // an element (other than via nested .find or .position calls on the
        // respective iterator), so just gonna do this manually ...
        for (i, j) in iproduct!(0..5, 0..5) {
            if board[[i,j]] == number {
                board[[i,j]] = -1;
                board[[i,5]] += 1;
                board[[5,j]] += 1;
                board[[5,5]] -= number;
                // println!("\nFound {} at ({}, {}) on board {}:\n{}", number, i, j, n, board);
                break;
            }
        }
    }
}

/// Checks whether there is a bingo on the given board
fn has_bingo(board: &Board) -> bool {
    board.slice(s![-1, 0..5]).iter().any(|&v| v == 5) ||
    board.slice(s![0..5, -1]).iter().any(|&v| v == 5)
}

/// Finds winners and keeps track of their scores in the winners map
///
/// Returns the board numbers of the newly winning boards
fn find_winners(number: i64, boards: &Vec<Board>, winners: &mut Winners) -> Vec<usize> {
    let mut new_winners = Vec::new();

    for (n, board) in boards.iter().enumerate() {
        if winners.contains_key(&n) {
            continue;
        }

        if has_bingo(&board) {
            new_winners.push(n);
            winners.insert(n, number * board[[5,5]]);
        }
    }

    return new_winners;
}

// -----------------------------------------------------------------------------

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let (numbers, mut boards) = read_numbers_and_boards(input);
    println!("Have {} numbers and {} boards.", numbers.len(), boards.len());

    // Go over numbers and mark them in the boards and looking for winners
    let mut winners = Winners::new();
    let mut new_winners = Vec::new();

    for (n, number) in numbers.iter().enumerate() {
        println!("Draw #{:2} yields:  {:2}", n, number);

        mark_boards(*number, &mut boards);
        new_winners = find_winners(*number, &boards, &mut winners);

        if winners.len() > 0 {
            println!("  Bingo! on boards:  {:?}", new_winners);
            break;
        }
    }

    if new_winners.len() != 1 {
        panic!("Expected single first winner, got: {:?}", new_winners);
    }

    return *winners.get(&new_winners[0]).unwrap() as i64;
}

/// Implements the solution for part 2
pub fn solve_part2(input: &String, _input_mode: &InputMode) -> i64 {
    let (numbers, mut boards) = read_numbers_and_boards(input);
    println!("Have {} numbers and {} boards.", numbers.len(), boards.len());

    // Go over numbers and mark them in the boards and looking for winners
    let mut winners = Winners::new();
    let mut new_winners = Vec::new();

    for (n, number) in numbers.iter().enumerate() {
        println!("Draw #{:2} yields:  {:2}", n, number);

        mark_boards(*number, &mut boards);
        new_winners = find_winners(*number, &boards, &mut winners);

        if new_winners.len() > 0 {
            println!("  new Bingo! on boards:  {:?}", new_winners);
        }
        if winners.len() == boards.len() {
            println!("\nAll bingo!");
            break;
        }
    }


    if new_winners.len() != 1 {
        panic!("Expected single last winner, got: {:?}", new_winners);
    }

    return *winners.get(&new_winners[0]).unwrap() as i64;
}
