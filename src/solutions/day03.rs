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

/// Reads multi-line string data into a vector of binary representations
fn read_into_binvec(input: &String) -> Vec<BinVec> {
    return input
        .lines()
        .map(|line| str2binvec(&line))
        .collect::<Vec<BinVec>>()
}

/// Counts the number of zero and one bits in a certain bit position
fn count_bits(data: &Vec<BinVec>, bit_pos: usize) -> (u32, u32) {
    let mut n0 = 0;
    for binary_num in data.iter() {
        if binary_num[bit_pos] == false {
            n0 += 1;
        }
    }
    return (n0, data.len() as u32 - n0);
}

/// Filters the data column-wise using a binary predicate
///
/// The binary predicate returns the value of the bit of those numbers that
/// are to be *kept*. More precisely: The numbers that have that bit in the
/// currently chosen bit position are kept and the others are discarded.
/// The binary predicate is called with the number of 0 and 1 bits.
fn filter_by_bit_pattern(data: &Vec<BinVec>, predicate: fn(u32, u32) -> bool) -> BinVec {
    let mut bit_pos = 0;
    let mut filtered = data.clone();

    println!("Filtering {} binary numbers by bit pattern ...", data.len());
    while filtered.len() > 1 {
        let (n0, n1) = count_bits(&filtered, bit_pos);
        let keep_bit = predicate(n0, n1);
        filtered.retain(|bin: &BinVec| bin[bit_pos] == keep_bit);

        println!("  Bit position {:2}:  {} entries left.", bit_pos, filtered.len());
        bit_pos += 1;
    }

    return filtered[0].to_vec();
}

// -----------------------------------------------------------------------------

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    let data = read_into_binvec(input);
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
    let data = read_into_binvec(input);
    let num_cols = data.get(0).unwrap().len();
    println!("Got {} binary numbers of width {}.", data.len(), num_cols);


    // Iterate over columns and filter by numbers with the most common bit in
    // the respective position
    let oxy_bin = filter_by_bit_pattern(&data, |n0: u32, n1: u32| n1 >= n0);
    let co2_bin = filter_by_bit_pattern(&data, |n0: u32, n1: u32| n0 > n1);

    let oxy_dec = bin2dec(&oxy_bin);
    let co2_dec = bin2dec(&co2_bin);
    println!("oxy:   {:?}  =  {}", oxy_bin, oxy_dec);
    println!("co2:   {:?}  =  {}", co2_bin, co2_dec);
    return oxy_dec * co2_dec;
}
