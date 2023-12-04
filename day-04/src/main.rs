#![feature(iterator_try_collect)]
use std::collections::BTreeMap;

use aoc_utils::{Error, Lines, Result};
use bit_set::BitSet;
use itertools::Itertools;

struct Day04;

fn parse_number_list(
    inp: &str,
) -> impl Iterator<Item = Result<usize, std::num::ParseIntError>> + '_ {
    inp.trim().split_ascii_whitespace().map(str::parse)
}

fn calculate_line_score(line: String) -> Result<usize> {
    let (_, relevant) = line
        .split_once(':')
        .ok_or_else(|| Error::input("Missing ':' delimiter"))?;
    let (first, second) = relevant
        .split_once('|')
        .ok_or_else(|| Error::input("Missing '|' delimiter"))?;
    let winning: BitSet = parse_number_list(first).try_collect()?;
    let numbers = parse_number_list(second)
        .filter_ok(|num| winning.contains(*num))
        .count();
    match numbers {
        0 => Ok(0),
        x => Ok(1 << (x - 1)),
    }
}

fn push_card_line_to_tree(
    mut tree: BTreeMap<usize, usize>,
    line: String,
) -> Result<BTreeMap<usize, usize>> {
    let (name, relevant) = line
        .split_once(':')
        .ok_or_else(|| Error::input("Missing ':' delimiter"))?;
    let card_id: usize = name
        .split_ascii_whitespace()
        .nth(1)
        .ok_or_else(|| Error::input("Missing card id"))?
        .parse()?;
    let (first, second) = relevant
        .split_once('|')
        .ok_or_else(|| Error::input("Missing '|' delimiter"))?;
    let winning: BitSet = parse_number_list(first).try_collect()?;
    let numbers = parse_number_list(second)
        .filter_ok(|num| winning.contains(*num))
        .count();
    *tree.entry(card_id).or_insert(0) += 1;
    match numbers {
        0 => Ok(tree),
        x => {
            let curr_count = tree.get(&card_id).copied().unwrap_or_default();
            for next in card_id + 1..=card_id + x {
                *tree.entry(next).or_insert(0) += curr_count;
            }
            Ok(tree)
        }
    }
}

impl aoc_utils::Problem<Lines> for Day04 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        input
            .map_ok(calculate_line_score)
            .try_fold(0_usize, |sum, res| match res {
                Ok(Ok(num)) => Ok(sum + num),
                Ok(Err(why)) => Err(why),
                Err(why) => Err(why.into()),
            })
    }

    fn solve_second(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        let solution = input
            .try_fold(BTreeMap::default(), |tree, res| match res {
                Ok(line) => push_card_line_to_tree(tree, line),
                Err(why) => Err(why.into()),
            })?
            .values()
            .sum();
        Ok(solution)
    }
}

aoc_utils::main!(Day04, Lines);
