use std::{fs::File, io::BufReader, num::ParseIntError};

use thiserror::Error;

#[macro_export]
macro_rules! main {
    ($problem:ident, $first_test_file:literal => $first_test_result:expr, $second_test_file:literal => $second_test_result:expr) => {
        fn main() -> aoc_utils::Result {
            use aoc_utils::Problem;
            $problem::run_with_input_output()
        }

        #[test]
        fn test_input_works() -> aoc_utils::Result {
            use aoc_utils::Problem;
            // First
            let file: &str = $first_test_file;
            let path = String::from("../inputs/") + file;
            let args = aoc_utils::Args::from_raw(aoc_utils::Variant::First, path);
            let exp = $first_test_result;
            let solution = $problem::solve(args);
            assert_eq!(exp, solution?);
            // Second
            let file: &str = $second_test_file;
            let path = String::from("../inputs/") + file;
            let args = aoc_utils::Args::from_raw(aoc_utils::Variant::Second, path);
            let exp = $second_test_result;
            let solution = $problem::solve(args);
            assert_eq!(exp, solution?);
            Ok(())
        }
    };
}

mod grid;
mod lines;

pub use grid::{Grid, Idx2D};
pub use lines::{InputLine, Lines, RawLine};

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {_0}")]
    Io(#[from] std::io::Error),
    #[error("Parsing a number")]
    ParseInt(#[from] ParseIntError),
    #[error("Invalid input: {_0:?}")]
    Input(String),
    #[error("Error: {_0}")]
    Other(String),
}

impl Error {
    pub fn input<S: Into<String>>(why: S) -> Self {
        Error::Input(why.into())
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub trait Problem<I: Input> {
    type Solution: std::fmt::Display;
    /// Run the solver with the given arguments.
    fn solve_first(input: I) -> Result<Self::Solution>;
    fn solve_second(input: I) -> Result<Self::Solution>;
    fn solve(args: Args) -> Result<Self::Solution> {
        match args.variant {
            Variant::First => Self::solve_first(I::from_args(args)?),
            Variant::Second => Self::solve_second(I::from_args(args)?),
        }
    }
    fn run_with_input_output() -> Result {
        let solution = Self::solve(Args {
            file: std::env::args().nth(1).unwrap_or_else(|| {
                eprintln!("> Reading from stdin..");
                String::from("-")
            }),
            variant: {
                let raw = std::env::args().nth(2).unwrap_or_else(|| {
                    eprintln!("> No variant given, solving first puzzle");
                    String::from("first")
                });
                match raw.as_str() {
                    "first" => Variant::First,
                    "second" => Variant::Second,
                    _ => panic!("Unknown variant {raw:?}"),
                }
            },
        })?;
        println!("{solution}");
        Ok(())
    }
}

pub trait Input
where
    Self: Sized,
{
    fn from_args(args: Args) -> Result<Self>;
}

pub struct Args {
    pub variant: Variant,
    pub file: String,
}

impl Args {
    pub fn from_raw(variant: Variant, file: String) -> Self {
        Args { variant, file }
    }
}

pub enum Variant {
    First,
    Second,
}

impl Input for Args {
    fn from_args(args: Args) -> Result<Self> {
        Ok(args)
    }
}

pub type Bytes = std::io::Bytes<BufReader<File>>;
impl Input for Bytes {
    fn from_args(args: Args) -> Result<Self> {
        let file = File::open(args.file)?;
        let reader = BufReader::new(file);
        Ok(std::io::Read::bytes(reader))
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!()
    }
}
