use std::collections::BTreeMap;

use aoc_utils::Lines;
use bit_set::BitSet;

struct Day04;

fn parse_number_list(inp: &str) -> impl Iterator<Item = usize> + '_ {
    inp.trim()
        .split_ascii_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
}

fn calculate_line_score(line: String) -> usize {
    let (_, relevant) = line.split_once(':').unwrap();
    let (first, second) = relevant.split_once('|').unwrap();
    let winning: BitSet = parse_number_list(first).collect();
    let numbers = parse_number_list(second)
        .filter(|num| winning.contains(*num))
        .count();
    match numbers {
        0 => 0,
        x => 1 << (x - 1),
    }
}

fn push_card_line_to_tree(
    mut tree: BTreeMap<usize, usize>,
    line: String,
) -> BTreeMap<usize, usize> {
    let (name, relevant) = line.split_once(':').unwrap();
    let card_id: usize = name
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let (first, second) = relevant.split_once('|').unwrap();
    let winning: BitSet = parse_number_list(first).collect();
    let numbers = parse_number_list(second)
        .filter(|num| winning.contains(*num))
        .count();
    *tree.entry(card_id).or_insert(0) += 1;
    match numbers {
        0 => tree,
        x => {
            let curr_count = tree.get(&card_id).copied().unwrap_or_default();
            for next in card_id + 1..=card_id + x {
                *tree.entry(next).or_insert(0) += curr_count;
            }
            tree
        }
    }
}

impl aoc_utils::Problem<Lines> for Day04 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let solution = input.map(Result::unwrap).map(calculate_line_score).sum();
        Ok(solution)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let solution = input
            .map(Result::unwrap)
            .fold(BTreeMap::default(), push_card_line_to_tree)
            .values()
            .sum();
        Ok(solution)
    }
}

aoc_utils::main!(Day04, Lines);
