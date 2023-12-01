use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum Lines {
    Stdin(std::io::Lines<std::io::StdinLock<'static>>),
    File(std::io::Lines<BufReader<File>>),
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

impl Iterator for Lines {
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Lines::Stdin(lines) => Iterator::next(lines),
            Lines::File(lines) => Iterator::next(lines),
        }
    }
}
