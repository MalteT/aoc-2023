#![feature(iter_map_windows)]

use aoc_utils::{Error, Lines, Result};
use fallible_iterator::FallibleIterator;

type Num = isize;

struct Day09;

impl aoc_utils::Problem<Lines> for Day09 {
    type Solution = Num;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        fallible_iterator::convert(input)
            .map_err(Error::from)
            .map(|s| parse_line(&s)?.collect())
            .map(find_next_in_sequence)
            .fold(0, |sum, num| Ok(sum + num))
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        fallible_iterator::convert(input)
            .map_err(Error::from)
            .map(|s| parse_line(&s)?.collect())
            .map(find_previous_in_sequence)
            .fold(0, |sum, num| Ok(sum + num))
    }
}

fn parse_line(input: &str) -> Result<impl Iterator<Item = Result<Num>> + '_> {
    Ok(input.split_ascii_whitespace().map(|slice| {
        let num = str::parse(slice)?;
        Ok(num)
    }))
}

fn find_next_in_sequence(readings: Vec<Num>) -> Result<Num> {
    let mut lasts = vec![];
    let mut curr_line = readings;
    loop {
        lasts.push(*curr_line.last().unwrap());
        match compute_non_zero_diff(&curr_line) {
            Some(next) => {
                curr_line = next;
            }
            None => {
                break;
            }
        }
    }
    Ok(lasts.into_iter().sum())
}

fn compute_non_zero_diff(line: &[isize]) -> Option<Vec<Num>> {
    let (found_non_zero, next) = line
        .iter()
        .map_windows(|[left, right]| *right - *left)
        .fold((false, vec![]), |(mut found_non_zero, mut next), num| {
            found_non_zero |= num != 0;
            next.push(num);
            (found_non_zero, next)
        });
    if found_non_zero {
        Some(next)
    } else {
        None
    }
}

fn find_previous_in_sequence(readings: Vec<Num>) -> Result<Num> {
    let mut firsts = vec![];
    let mut curr_line = readings;
    loop {
        firsts.push(curr_line[0]);
        match compute_non_zero_diff(&curr_line) {
            Some(next) => {
                curr_line = next;
            }
            None => break,
        }
    }
    let solution = firsts
        .into_iter()
        .rev()
        .reduce(|right, left| left - right)
        .unwrap();
    Ok(solution)
}

aoc_utils::main!(Day09, "inputs-09-test" => 114, "inputs-09-test" => 2);
