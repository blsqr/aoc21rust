//! # Solutions for Day 05 - Hydrothermal Venture
//!
//! For puzzle text, see https://adventofcode.com/2021/day/5

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::fmt;

#[allow(unused_imports)]
use ndarray::prelude::*;

use crate::InputMode;

type Seafloor = ndarray::Array2<i64>;

/// A point, i.e. coordinates in the x-y seafloor plane
#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

/// A line on the seafloor, i.e. a combination of source and destination points
///
/// Implementation includes first steps in building structs and methods to
/// solve this problem ... perhaps a bit overkill for this, though
#[derive(Debug)]
struct Line {
    src: Point,
    dest: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.src.y == self.dest.y
    }

    fn is_vertical(&self) -> bool {
        self.src.x == self.dest.x
    }

    fn is_diagonal(&self) -> bool {
        (self.src.x as i64 - self.dest.x as i64).abs()
            == (self.src.y as i64 - self.dest.y as i64).abs()
    }

    /// Iterates over x-coordinates of this line from source to destination
    ///
    /// Uses type erasure to allow forward or reverse iteration
    fn x_iter(&self) -> Box<dyn Iterator<Item = usize>> {
        if self.src.x <= self.dest.x {
            return Box::new(self.src.x..=self.dest.x) as Box<dyn Iterator<Item = usize>>;
        }
        return Box::new((self.dest.x..=self.src.x).rev());
    }

    /// Iterates over y-coordinates of this line from source to destination
    ///
    /// Uses type erasure to allow forward or reverse iteration
    fn y_iter(&self) -> Box<dyn Iterator<Item = usize>> {
        if self.src.y <= self.dest.y {
            return Box::new(self.src.y..=self.dest.y) as Box<dyn Iterator<Item = usize>>;
        }
        return Box::new((self.dest.y..=self.src.y).rev());
    }

    /// Returns the points between source and destination (inclusive)
    fn points(&self, incl_diag: bool) -> Vec<Point> {
        let mut pts = Vec::new();

        if self.is_horizontal() {
            for x in self.x_iter() {
                pts.push(Point {
                    x: x,
                    y: self.src.y,
                });
            }
        } else if self.is_vertical() {
            for y in self.y_iter() {
                pts.push(Point {
                    x: self.src.x,
                    y: y,
                });
            }
        } else if self.is_diagonal() {
            if incl_diag {
                // println!("Line {}:", self);
                for (x, y) in self.x_iter().zip(self.y_iter()) {
                    // println!("  ({}, {})", x, y);
                    pts.push(Point { x: x, y: y });
                }
            }
            // else: do not add any points
        } else {
            panic!("Line {} is not horizontal, vertical, or diagonal!", self);
        }

        return pts;
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.src, self.dest)
    }
}
// -----------------------------------------------------------------------------

fn parse_line(line: &str) -> Line {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    }
    let cap = PATTERN.captures(line).unwrap();
    Line {
        src: Point {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
        },
        dest: Point {
            x: cap[3].parse().unwrap(),
            y: cap[4].parse().unwrap(),
        },
    }
}

fn mark_lines(lines: &Vec<Line>, seafloor: &mut Seafloor, incl_diag: bool) {
    for line in lines {
        for pt in line.points(incl_diag) {
            seafloor[[pt.y, pt.x]] += 1;
        }
        // println!("Marked line {} ... Seafloor is now:\n{}\n", line, seafloor);
    }
}

/// Marks
fn mark_seafloor(lines: &Vec<Line>, incl_diag: bool, _input_mode: &InputMode) -> Seafloor {
    // Find the size of the seafloor
    let mut max_coord = 0;
    for line in lines {
        if matches!(_input_mode, InputMode::Test) {
            println!("  {}", line);
        }
        max_coord = max(
            max_coord,
            max(line.src.x, max(line.src.y, max(line.dest.x, line.dest.y))),
        );
    }

    // Construct the seafloor map to a suitable size
    let mut seafloor = Seafloor::zeros((max_coord + 1, max_coord + 1));
    println!(
        "Have {} lines on seafloor of shape {:?} ...",
        lines.len(),
        seafloor.shape()
    );

    // Now mark the lines on the seafloor
    println!("Now marking lines (incl_diag: {}) ...", incl_diag);
    mark_lines(&lines, &mut seafloor, incl_diag);

    if matches!(_input_mode, InputMode::Test) {
        println!("Seafloor is now marked:\n{}", seafloor);
    }
    return seafloor;
}

// -----------------------------------------------------------------------------

/// Implements the solution for part 1
pub fn solve_part1(input: &String, _input_mode: &InputMode) -> i64 {
    println!("Parsing lines ...");
    let lines = input.lines().map(parse_line).collect::<Vec<Line>>();
    mark_seafloor(&lines, false, _input_mode)
        .iter()
        .filter(|&h| *h >= 2)
        .count() as i64
}

/// Implements the solution for part 2
pub fn solve_part2(input: &String, _input_mode: &InputMode) -> i64 {
    println!("Parsing lines ...");
    let lines = input.lines().map(parse_line).collect::<Vec<Line>>();
    mark_seafloor(&lines, true, _input_mode)
        .iter()
        .filter(|&h| *h >= 2)
        .count() as i64
}
