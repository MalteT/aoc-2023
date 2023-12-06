#![feature(isqrt)]
use aoc_utils::{Error, Lines};

struct Day06;

impl aoc_utils::Problem<Lines> for Day06 {
    type Solution = usize;

    fn solve_first(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        let first = input
            .next()
            .ok_or_else(|| Error::input("Where is the first line?"))??;
        let second = input
            .next()
            .ok_or_else(|| Error::input("Where is the second line?"))??;
        let first = first
            .split_ascii_whitespace()
            .skip(1)
            .map(str::parse::<usize>);
        let second = second
            .split_ascii_whitespace()
            .skip(1)
            .map(str::parse::<usize>);
        first
            .zip(second)
            .try_fold(1_usize, |product, (total_time, winning_distance)| {
                let total_time = total_time?;
                let winning_distance = winning_distance?;
                // Could be 0.5 too small
                let hodl_end = (total_time.pow(2).saturating_sub(4 * winning_distance)).isqrt() / 2;
                // Could be 0.5 too small
                let hodl_start = total_time / 2;
                // Could be 0.5 too small or too large
                let hodl_min = hodl_start - hodl_end;
                // Could be 1.0 too small
                let hodl_max = hodl_start + hodl_end;
                // Could be 1.5 too small or 0.5 too large
                let mut ways_to_win = hodl_max - hodl_min + 1;
                if distance(hodl_min, total_time) <= winning_distance {
                    ways_to_win -= 1
                }
                if distance(hodl_max, total_time) <= winning_distance {
                    ways_to_win -= 1
                }
                if hodl_min > 0 && distance(hodl_min - 1, total_time) > winning_distance {
                    ways_to_win += 1
                }
                if distance(hodl_max + 1, total_time) > winning_distance {
                    ways_to_win += 1
                }
                if distance(hodl_max + 2, total_time) > winning_distance {
                    ways_to_win += 1
                }
                Ok(product * ways_to_win)
            })
    }

    fn solve_second(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        let first = input
            .next()
            .ok_or_else(|| Error::input("Where is the first line?"))??;
        let second = input
            .next()
            .ok_or_else(|| Error::input("Where is the second line?"))??;
        let total_time = first
            .bytes()
            .filter(|byte| byte.is_ascii_digit())
            .fold(0_usize, |num, digit| num * 10 + (digit - b'0') as usize);
        let winning_distance = second
            .bytes()
            .filter(|byte| byte.is_ascii_digit())
            .fold(0_usize, |num, digit| num * 10 + (digit - b'0') as usize);
        // Could be 0.5 too small
        let hodl_end = (total_time.pow(2).saturating_sub(4 * winning_distance)).isqrt() / 2;
        // Could be 0.5 too small
        let hodl_start = total_time / 2;
        // Could be 0.5 too small or too large
        let hodl_min = hodl_start - hodl_end;
        // Could be 1.0 too small
        let hodl_max = hodl_start + hodl_end;
        // Could be 1.5 too small or 0.5 too large
        let mut ways_to_win = hodl_max - hodl_min + 1;
        if distance(hodl_min, total_time) <= winning_distance {
            ways_to_win -= 1
        }
        if distance(hodl_max, total_time) <= winning_distance {
            ways_to_win -= 1
        }
        if hodl_min > 0 && distance(hodl_min - 1, total_time) > winning_distance {
            ways_to_win += 1
        }
        if distance(hodl_max + 1, total_time) > winning_distance {
            ways_to_win += 1
        }
        if distance(hodl_max + 2, total_time) > winning_distance {
            ways_to_win += 1
        }
        Ok(ways_to_win)
    }
}

fn distance(hodl: usize, total_time: usize) -> usize {
    hodl * (total_time.saturating_sub(hodl))
}

aoc_utils::main!(Day06, Lines, "inputs-06-test" => 288, "inputs-06-test" => 71503);
