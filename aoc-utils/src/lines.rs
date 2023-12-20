use std::{
    fs::File,
    io::{BufRead, BufReader},
    marker::PhantomData,
};

use crate::{Args, Input, Result};

pub type InputLine = Result<String, std::io::Error>;

pub struct Lines<T = InputLine>
where
    T: From<InputLine>,
{
    iter: Iter,
    _t: PhantomData<T>,
}

pub enum Iter {
    Stdin(std::io::Lines<std::io::StdinLock<'static>>),
    File(std::io::Lines<BufReader<File>>),
}

impl<T> Iterator for Lines<T>
where
    T: From<InputLine>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match &mut self.iter {
            Iter::Stdin(iter) => iter.next(),
            Iter::File(iter) => iter.next(),
        };
        next.map(T::from)
    }
}

impl<T> Input for Lines<T>
where
    T: From<InputLine>,
{
    fn from_args(args: Args) -> Result<Self> {
        if args.file == "-" {
            let stdin = std::io::stdin();
            Ok(Lines {
                iter: Iter::Stdin(stdin.lines()),
                _t: PhantomData,
            })
        } else {
            let file = File::open(args.file)?;
            let reader = BufReader::new(file).lines();
            Ok(Lines {
                iter: Iter::File(reader),
                _t: PhantomData,
            })
        }
    }
}

pub struct RawLine(pub String);

impl RawLine {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<InputLine> for RawLine {
    fn from(line: InputLine) -> Self {
        RawLine(line.expect("Error in input, line not readable"))
    }
}

impl std::ops::Deref for RawLine {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
