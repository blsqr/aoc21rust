//! CLI for invoking the puzzle solution function for a desired day and part

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use utils::InputMode;

mod solutions;
mod utils;

/// Type alias for solution functions
pub type SolutionFunc = fn(&String, &InputMode) -> i64;

/// Constructs a map of all available solution functions
fn get_solution_functions() -> HashMap<(u8, u8), SolutionFunc> {
    let mut funcs: HashMap<(u8, u8), SolutionFunc> = HashMap::new();

    funcs.insert((1, 1), solutions::day01::solve_part1);
    funcs.insert((1, 2), solutions::day01::solve_part2);

    funcs.insert((2, 1), solutions::day02::solve_part1);
    funcs.insert((2, 2), solutions::day02::solve_part2);

    funcs.insert((3, 1), solutions::day03::solve_part1);
    funcs.insert((3, 2), solutions::day03::solve_part2);

    funcs.insert((4, 1), solutions::day04::solve_part1);
    funcs.insert((4, 2), solutions::day04::solve_part2);

    funcs.insert((5, 1), solutions::day05::solve_part1);
    funcs.insert((5, 2), solutions::day05::solve_part2);

    funcs.insert((6, 1), solutions::day06::solve_part1);
    funcs.insert((6, 2), solutions::day06::solve_part2);

    funcs.insert((7, 1), solutions::day07::solve_part1);
    funcs.insert((7, 2), solutions::day07::solve_part2);

    funcs.insert((8, 1), solutions::day08::solve_part1);
    funcs.insert((8, 2), solutions::day08::solve_part2);

    return funcs;
}

/// Load the input file as an unprocessed string
fn load_input(day: u8, _part: u8, input_mode: &InputMode, input_dir: &str) -> String {
    let cwd = &env::current_dir().unwrap();
    let dir = Path::new(cwd).join(input_dir);
    let filepath = match input_mode {
        InputMode::Test => dir.join(format!("day{:02}_test.txt", day)),
        InputMode::Full => dir.join(format!("day{:02}.txt", day)),
    };

    println!("Loading input from:\n  {:?}", &filepath);
    let input = fs::read_to_string(filepath).expect("Failed reading input file!");
    println!(
        "Loaded input. (Length: {}, Lines: {})",
        input.len(),
        input.matches("\n").count()
    );

    return input;
}

/// Provide the CLI for invoking Advent of Code 2021 solution functions
fn main() {
    println!("\n--- Advent of Code 2021 ---");

    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Invalid number of arguments! Need: [day] [part] [--test/--full].");
    }

    let day = args[1].parse::<u8>().unwrap();
    let part = args[2].parse::<u8>().unwrap();
    let input_mode = match &args[3] as &str {
        "--test" => InputMode::Test,
        "--full" => InputMode::Full,
        _ => panic!(
            "Invalid input mode {:?}, should be --test or --full!",
            &args[3]
        ),
    };
    let input_dir = "input";

    println!("---- Day {:02} --- Part {} ----\n", day, part);
    let input = load_input(day, part, &input_mode, &input_dir);

    // Retrieve and invoke solution function
    let solution = match get_solution_functions().get(&(day, part)) {
        Some(func) => {
            println!("\nNow computing solution ...");
            func(&input, &input_mode)
        }
        None => panic!("No solution function registered for this day or part!"),
    };

    println!("The solution is:  {}", solution);
}
