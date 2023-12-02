use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum Lines {
    Stdin(std::io::Lines<std::io::StdinLock<'static>>),
    File(std::io::Lines<BufReader<File>>),
}

pub enum Variant {
    First,
    Second,
}

pub struct Args {
    pub variant: Variant,
    pub file: String,
}

pub fn open_by_lines<S: AsRef<str>>(file: S) -> std::io::Result<Lines> {
    let file = file.as_ref();
    if file == "-" {
        let stdin = std::io::stdin();
        Ok(Lines::Stdin(stdin.lines()))
    } else {
        let file = File::open(file)?;
        let reader = BufReader::new(file).lines();
        Ok(Lines::File(reader))
    }
}

pub fn parse_args() -> Args {
    Args {
        file: first_arg_as_file_name(),
        variant: second_arg_as_variant(),
    }
}

pub fn second_arg_as_variant() -> Variant {
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

pub fn first_arg_as_file_name() -> String {
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
