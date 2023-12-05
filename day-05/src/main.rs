#![feature(iter_array_chunks, iter_collect_into)]
use std::ops::Range;

use aoc_utils::{Error, Lines, Result};

type Seed = usize;
type RangeList = Vec<Range<usize>>;

struct Day05;

#[derive(Debug)]
enum Map {
    Add(usize),
    Sub(usize),
}

impl Map {
    fn map_range(&self, Range { start, end }: Range<usize>) -> Range<usize> {
        match *self {
            Map::Add(offset) => {
                let start = start.saturating_add(offset);
                let end = end.saturating_add(offset);
                start..end
            }
            Map::Sub(offset) => {
                let start = start.saturating_sub(offset);
                let end = end.saturating_sub(offset);
                start..end
            }
        }
    }
}

#[derive(Debug)]
enum Line {
    Empty,
    Header,
    Map { from: Range<usize>, apply: Map },
}

impl aoc_utils::Problem<Lines> for Day05 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        solve_helper(input, parse_seed_number_line)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        solve_helper(input, parse_seed_range_line)
    }
}

fn solve_helper<F>(mut input: Lines, seed_aggregator: F) -> Result<usize>
where
    F: FnOnce(String) -> Result<RangeList>,
{
    let seeds = seed_aggregator(
        input
            .next()
            .ok_or_else(|| Error::input("Not enough lines"))??,
    )?;
    let seeds_len = seeds.len();
    let (mut last, mut curr) = input
        .map(Line::parse)
        .try_fold((seeds, Vec::with_capacity(seeds_len)), apply_line)?;
    last.drain(..).collect_into(&mut curr);
    curr.into_iter()
        .map(|range| range.start)
        .min()
        .ok_or_else(|| Error::input("No seeds?"))
}

fn apply_line(
    (mut last, mut curr): (RangeList, RangeList),
    line: Result<Line>,
) -> Result<(RangeList, RangeList)> {
    match line? {
        Line::Empty => Ok((last, curr)),
        Line::Header => {
            last.drain(..).collect_into(&mut curr);
            Ok((curr, Vec::with_capacity(last.len())))
        }
        Line::Map { from, apply } => {
            let mut idx = 0;
            // TODO: don't iterate over new entries
            while idx < last.len() {
                let entry = &last[idx];
                use std::cmp::Ordering::*;
                match (entry.start.cmp(&from.start), entry.end.cmp(&from.end)) {
                    (Equal | Greater, Equal | Less) => {
                        // Completely inside the from range
                        let entry = last.remove(idx);
                        curr.push(apply.map_range(entry.start..entry.end))
                    }
                    (Equal | Less, Equal | Greater) => {
                        // The from is completely inside the entry
                        let entry = last.remove(idx);
                        [(entry.start..from.start), (from.end..entry.end)]
                            .into_iter()
                            .filter(|range| !range.is_empty())
                            .collect_into(&mut last);
                        curr.push(apply.map_range(from.start..from.end));
                    }
                    (Less, Less) if entry.end > from.start => {
                        // The right half of entry overlaps with from's left half
                        let entry = last.remove(idx);
                        last.push(entry.start..from.start);
                        curr.push(apply.map_range(from.start..entry.end));
                    }
                    (Greater, Greater) if from.end > entry.start => {
                        // The left half of entry overlaps with from's right half
                        let entry = last.remove(idx);
                        curr.push(apply.map_range(entry.start..from.end));
                        last.push(from.end..entry.end);
                    }
                    _ => idx += 1,
                }
            }
            Ok((last, curr))
        }
    }
}

impl Line {
    fn parse(line: std::io::Result<String>) -> Result<Self> {
        let input = line?;
        if input.is_empty() {
            Ok(Self::Empty)
        } else if input.bytes().next().map(|byte| byte.is_ascii_digit()) == Some(true) {
            let mut numbers = input.split_ascii_whitespace().map(parse_number);
            let to_start = numbers
                .next()
                .ok_or_else(|| Error::input("Not enough numbers on line"))??;
            let from_start = numbers
                .next()
                .ok_or_else(|| Error::input("Not enough numbers on line"))??;
            let len = numbers
                .next()
                .ok_or_else(|| Error::input("Not enough numbers on line"))??;
            Ok(Line::Map {
                from: from_start..from_start + len,
                apply: if from_start > to_start {
                    Map::Sub(from_start - to_start)
                } else {
                    Map::Add(to_start - from_start)
                },
            })
        } else {
            Ok(Self::Header)
        }
    }
}

fn parse_number(input: &str) -> Result<usize> {
    input.parse().map_err(Error::from)
}

fn parse_seed_range_line(input: String) -> Result<Vec<Range<Seed>>> {
    parse_seed_line_helper(&input)?
        .array_chunks()
        .map(|[start, len]| {
            let start: usize = start?;
            Ok(start..start + len?)
        })
        .collect()
}

fn parse_seed_number_line(input: String) -> Result<Vec<Range<Seed>>> {
    parse_seed_line_helper(&input)?
        .map(|seed| {
            let seed: usize = seed?;
            Ok(seed..seed + 1)
        })
        .collect()
}

fn parse_seed_line_helper(input: &str) -> Result<impl Iterator<Item = Result<usize>> + '_> {
    Ok(input
        .strip_prefix("seeds: ")
        .ok_or_else(|| Error::input("invalid seeds line"))?
        .split_ascii_whitespace()
        .map(|raw| str::parse(raw).map_err(Error::from)))
}

aoc_utils::main!(Day05, Lines, "inputs-05-test" => 35, "inputs-05-test" => 46);
