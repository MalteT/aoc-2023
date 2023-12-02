use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[macro_export]
macro_rules! main {
    ($problem:ident, $input:ident) => {
        fn main() -> aoc_utils::Result {
            <$problem as aoc_utils::Solver<aoc_utils::$input>>::solve()
        }
    };
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {_0}")]
    Io(#[from] std::io::Error),
    #[error("Error: {_0}")]
    Other(String),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub trait Problem<Input> {
    type Solution;
    /// Run the solver with the given arguments.
    fn solve_first(input: Input) -> Result<Self::Solution>;
    fn solve_second(input: Input) -> Result<Self::Solution>;
}

pub trait Input
where
    Self: Sized,
{
    fn from_args(args: Args) -> Result<Self>;
}

pub trait Solver<I> {
    fn solve() -> Result;
}

pub struct Args {
    pub variant: Variant,
    pub file: String,
}

pub enum Lines {
    Stdin(std::io::Lines<std::io::StdinLock<'static>>),
    File(std::io::Lines<BufReader<File>>),
}

pub enum Variant {
    First,
    Second,
}

fn parse_args() -> Args {
    Args {
        file: first_arg_as_file_name(),
        variant: second_arg_as_variant(),
    }
}

fn second_arg_as_variant() -> Variant {
    let raw = std::env::args().nth(2).unwrap_or_else(|| {
        eprintln!("> No variant given, solving first puzzle");
        String::from("first")
    });
    match raw.as_str() {
        "first" => Variant::First,
        "second" => Variant::Second,
        _ => panic!("Unknown variant {raw:?}"),
    }
}

fn first_arg_as_file_name() -> String {
    std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("> Reading from stdin..");
        String::from("-")
    })
}

impl Iterator for Lines {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Lines::Stdin(lines) => Iterator::next(lines),
            Lines::File(lines) => Iterator::next(lines),
        }
    }
}

impl<O: std::fmt::Display, I: Input, T: Problem<I, Solution = O>> Solver<I> for T {
    fn solve() -> Result {
        let args = parse_args();
        let output = match args.variant {
            Variant::First => T::solve_first(I::from_args(args)?),
            Variant::Second => T::solve_second(I::from_args(args)?),
        }?;
        println!("{output}");
        Ok(())
    }
}

impl Input for Lines {
    fn from_args(args: Args) -> Result<Self> {
        if args.file == "-" {
            let stdin = std::io::stdin();
            Ok(Lines::Stdin(stdin.lines()))
        } else {
            let file = File::open(args.file)?;
            let reader = BufReader::new(file).lines();
            Ok(Lines::File(reader))
        }
    }
}

impl Input for Args {
    fn from_args(args: Args) -> Result<Self> {
        Ok(args)
    }
}
