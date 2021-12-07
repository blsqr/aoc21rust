//! # Solutions for Day 03 - Binary Diagnostic

use crate::InputMode;

pub type BinVec = Vec<bool>;

/// Converts a string of 0 and 1 to a BinVec
fn str2binvec(s: &str) -> BinVec {
    let mut binvec = BinVec::new();

    for c in s.chars() {
        match c {
            '0' => binvec.push(false),
            '1' => binvec.push(true),
            _ => panic!("Invalid character: {:?}", c),
        }
    }

    return binvec;
}

/// Converts a binary value (encoded as BinVec) to a decimal value
fn bin2dec(vec: &BinVec) -> i64 {
    let mut dec: i64 = 0;
    for (n, bit) in vec.iter().rev().enumerate() {
        match bit {
            false => {}
            true => dec += 2_i64.pow(n as u32),
        }
    }
    return dec;
}

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let data = input
        .lines()
        .map(|line| str2binvec(&line))
        .collect::<Vec<BinVec>>();

    // Find number of columns
    let num_cols = data.get(0).unwrap().len();
    println!("Got {} binary numbers of width {}.", data.len(), num_cols);

    // Go over columns and count most frequent bit --> gamma rate
    let mut gamma_bin = BinVec::new();

    let mut num_true_bits;
    for bit_pos in 0..num_cols {
        num_true_bits = 0;
        for binary_num in data.iter() {
            match binary_num[bit_pos] {
                true => num_true_bits += 1,
                false => {}
            }
        }
        gamma_bin.push(num_true_bits > data.len() / 2);
    }

    // For the epsilon rate, use the inverse of the gamma rate (least frequent bit)
    let mut epsilon_bin = BinVec::new();
    for bit in gamma_bin.iter() {
        epsilon_bin.push(!bit);
    }

    // Turn into decimal
    let gamma_dec = bin2dec(&gamma_bin);
    let epsilon_dec = bin2dec(&epsilon_bin);
    println!("Gamma:   {:?}  =  {}", gamma_bin, gamma_dec);
    println!("Epsilon: {:?}  =  {}", epsilon_bin, epsilon_dec);
    return gamma_dec * epsilon_dec;
}

/// Implements the solution for part 2
pub fn solve_part2(input: &String, _input_mode: &InputMode) -> i64 {
    0
}
