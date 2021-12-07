//! Some utility type and function definitions

/// A selector for the various input modes to a puzzle solution function
pub enum InputMode {
    Full,
    Test,
}

/// Type alias for solution functions
pub type SolutionFunc = fn(&String) -> i64;
