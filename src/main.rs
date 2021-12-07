// CLI for invoking the puzzle solution function for a desired day and part

use std::env;
use std::path::Path;
use std::fs;
use std::collections::HashMap;

use utils::InputMode;

mod solutions;
mod utils;

/// Returns the hashmap of all available solution functions
fn register_solution_functions() -> HashMap<(u8, u8), fn(&String) -> i64> {

    let mut funcs: HashMap<(u8, u8), fn(&String) -> i64> = HashMap::new();

    funcs.insert((1, 1), solutions::day01::solve_part1);

    return funcs
}

/// Load the input file as a sequence of strings
fn load_input(day: u8, input_mode: &InputMode, input_dir: &str) -> String {
    let cwd = &env::current_dir().unwrap();
    let dir = Path::new(cwd).join(input_dir);
    let filepath = match input_mode {
        InputMode::Test => dir.join(format!("day{:02}_test.txt", day)),
        InputMode::Full => dir.join(format!("day{:02}.txt", day)),
    };

    println!("Loading input from:\n  {:?}", &filepath);
    fs::read_to_string(filepath).expect("Failed reading input file!")
}


fn main() {
    println!("\n--- Advent of Code 2021 ---");

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Invalid number of arguments! Provide only [day] [part].");
    }

    let day = args[1].parse::<u8>().unwrap();
    let part = args[2].parse::<u8>().unwrap();
    let input_mode = InputMode::Test;
    let input_dir = "input";

    println!("---- Day {:02} --- Part {} ----\n", day, part);
    let input = load_input(day, &input_mode, &input_dir);

    // Retrieve and invoke solution function
    let funcs = register_solution_functions();

    println!("\nNow computing solution ...");
    let solution = match funcs.get(&(day, part)) {
        Some(func) => func(&input),
        None => -1,
    };

    println!("The solution is:  {}", solution);
}