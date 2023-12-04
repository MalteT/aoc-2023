use std::collections::BTreeMap;

use aoc_utils::Lines;
use bit_set::BitSet;

struct Day04;

impl aoc_utils::Problem<Lines> for Day04 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let solution = input
            .map(Result::unwrap)
            .map(|line| {
                let (_, relevant) = line.split_once(':').unwrap();
                let (first, second) = relevant.split_once('|').unwrap();
                let winning: BitSet = first
                    .trim()
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect();
                let numbers = second
                    .trim()
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .filter(|num| winning.contains(*num))
                    .count();
                match numbers {
                    0 => 0,
                    x => 1 << (x - 1),
                }
            })
            .sum();
        Ok(solution)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let solution = input
            .map(Result::unwrap)
            .fold(BTreeMap::<usize, usize>::new(), |mut acc, line| {
                let (name, relevant) = line.split_once(':').unwrap();
                let card_id: usize = name
                    .split_ascii_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                let (first, second) = relevant.split_once('|').unwrap();
                let winning: BitSet = first
                    .trim()
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect();
                let numbers = second
                    .trim()
                    .split_ascii_whitespace()
                    .map(str::parse)
                    .map(Result::unwrap)
                    .filter(|num| winning.contains(*num))
                    .count();
                *acc.entry(card_id).or_insert(0) += 1;
                match numbers {
                    0 => acc,
                    x => {
                        let curr_count = acc.get(&card_id).copied().unwrap_or_default();
                        for next in card_id + 1..=card_id + x {
                            *acc.entry(next).or_insert(0) += curr_count;
                        }
                        acc
                    }
                }
            })
            .values()
            .sum();
        Ok(solution)
    }
}

aoc_utils::main!(Day04, Lines);
