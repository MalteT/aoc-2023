#![feature(extract_if)]
use std::{collections::BTreeMap, mem, ops::Range};

use aoc_utils::{Bytes, Result};
use bit_set::BitSet;
use itertools::Itertools;

struct Day03;
type Position = usize;

#[derive(Debug)]
enum Token {
    Number(Range<Position>, usize),
    Symbol(Position),
    Gear(Position),
    Newline,
}

/// The various states of a gear during accumulation
#[derive(Debug, Default)]
enum Gear {
    /// No neighbor found
    #[default]
    None,
    // A single neighbor found
    One(usize),
    // Two neighbors! YEAH!
    Two(usize),
    // More than two...
    Invalid,
}

impl Gear {
    // Add a new neighbor to this gear
    fn register_neighbor(&mut self, num: usize) {
        *self = match self {
            Gear::None => Gear::One(num),
            Gear::One(curr) => Gear::Two(*curr * num),
            Gear::Two(_) => Gear::Invalid,
            Gear::Invalid => Gear::Invalid,
        }
    }
    /// Convert this gear into it's gear ratio
    fn get_ratio(&self) -> usize {
        match self {
            Gear::None => 0,
            Gear::One(_) => 0,
            Gear::Two(num) => *num,
            Gear::Invalid => 0,
        }
    }
}

/// Accumulator for the first problem
#[derive(Debug, Default)]
struct SimpleAcc {
    /// The total sum of numbers found that neighbor a symbol
    sum: usize,
    /// Symbols found on this and the last line
    symbols: LastAndCurr<BitSet>,
    /// Numbers that are not yet applied
    /// These will be removed as soon as a neighboring symbol is found.
    /// The range spans two additional tiles at the start and at the end!
    numbers: LastAndCurr<Vec<(Range<usize>, usize)>>,
}

/// Helper containing information about the current and the last line
/// The contained type needs [`Default`] to be created when a new line is found
#[derive(Debug, Default)]
struct LastAndCurr<T>
where
    T: Default,
{
    pub last: T,
    pub curr: T,
}

impl<T> LastAndCurr<T>
where
    T: Default,
{
    /// A newline was found!
    /// Swap the stored lines and replace the current one with a fresh one
    pub fn push_newline(&mut self) {
        mem::swap(&mut self.last, &mut self.curr);
        self.curr = T::default();
    }
}

/// Accumulator for the second problem
#[derive(Debug, Default)]
struct GearAcc {
    /// Total sum of gear ratios
    sum: usize,
    /// Gears on this and the last line, complete with position information
    gears: LastAndCurr<BTreeMap<Position, Gear>>,
    /// Numbers on this and the last line, with the range they're spanning.
    /// The range spans two additional tiles at the start and at the end!
    numbers: LastAndCurr<Vec<(Range<Position>, usize)>>,
}

/// A scanner for converting raw bytes into usable input
#[derive(Debug, Default)]
struct Scanner {
    /// The position inside a line we're currently in
    line_position: usize,
    /// Current number + position of the first digit of the
    /// number we're currently scanning, [`None`] if we're not.
    number: Option<(usize, Position)>,
}

impl aoc_utils::Problem<Bytes> for Day03 {
    type Solution = usize;

    fn solve_first(input: Bytes) -> aoc_utils::Result<Self::Solution> {
        let acc = input
            .scan(Scanner::default(), scan_input)
            .flatten_ok()
            .try_fold(SimpleAcc::default(), fold_entries_around_symbols);
        Ok(acc?.sum)
    }

    fn solve_second(input: Bytes) -> aoc_utils::Result<Self::Solution> {
        let acc = input
            .scan(Scanner::default(), scan_input)
            .flatten_ok()
            .try_fold(GearAcc::default(), fold_entries_around_gears);
        Ok(acc?.sum)
    }
}

/// Scan a single byte with our [`Scanner`].
///
/// There are a number of possible bytes we could scan here:
/// - `0..=9`, part of a larger number that we'll accumulate
/// - `.`, the empty tile, we'll pop any number we've accumulated so far
/// - `*`, a gear, pop the accumulated number and produce a gear
/// - `\n`, a newline, pop the accumulated number and produce a newline
/// - everything else will pop the number and produce a symbol marker
fn scan_input(acc: &mut Scanner, byte: std::io::Result<u8>) -> Option<Result<Vec<Token>>> {
    let mut pop_scanned_number = || match acc.number.take() {
        Some((num, pos)) => vec![Token::Number(
            pos.saturating_sub(1)..acc.line_position + 1,
            num,
        )],
        None => vec![],
    };
    match byte {
        Ok(byte) => {
            let res = match byte {
                b'0'..=b'9' => {
                    match acc.number.as_mut() {
                        Some((num, _)) => {
                            *num *= 10;
                            *num += (byte - b'0') as usize;
                        }
                        None => {
                            acc.number = Some(((byte - b'0') as usize, acc.line_position));
                        }
                    }
                    acc.line_position += 1;
                    vec![]
                }
                b'.' => {
                    let res = pop_scanned_number();
                    acc.line_position += 1;
                    res
                }
                b'*' => {
                    let mut res = pop_scanned_number();
                    res.push(Token::Gear(acc.line_position));
                    acc.line_position += 1;
                    res
                }
                b'\n' => {
                    let mut res = pop_scanned_number();
                    res.push(Token::Newline);
                    acc.line_position = 0;
                    res
                }
                _ => {
                    let mut res = pop_scanned_number();
                    res.push(Token::Symbol(acc.line_position));
                    acc.line_position += 1;
                    res
                }
            };
            Some(Ok(res))
        }
        Err(why) => Some(Err(why.into())),
    }
}

/// Main part of the first problem.
///
/// Assuming any given [`SimpleAcc`] and an [`Token`], do the following depending on the type of token:
/// - **Number**: Look for symbols in the last and current line that are adjacent to the number. If any is found, add it to the sum. If none is found, we'll keep track of the number and it's position
/// - **Symbol**/**Gear**: Look for numbers in the last and current line that are adjacent to the symbol. If any is found, pop it from the list and add it to the sum
/// - **Newline**: Move the current line to the last and start with a fresh current line. Numbers and symbols from the last line will be dropped as they are not needed anymore
fn fold_entries_around_symbols(mut acc: SimpleAcc, token: Result<Token>) -> Result<SimpleAcc> {
    match token? {
        Token::Number(range, num) => {
            let symbol_on_last_line = range.clone().any(|pos| acc.symbols.last.contains(pos));
            if symbol_on_last_line || acc.symbols.curr.contains(range.start) {
                acc.sum += num;
            } else {
                acc.numbers.curr.push((range, num))
            }
            Ok(acc)
        }
        Token::Symbol(pos) | Token::Gear(pos) => {
            let mut idx = 0;
            while idx < acc.numbers.last.len() {
                if acc.numbers.last[idx].0.contains(&pos) {
                    let (_, num) = acc.numbers.last.remove(idx);
                    acc.sum += num;
                } else {
                    idx += 1;
                }
            }
            if let Some((range, num)) = acc.numbers.curr.last() {
                if range.contains(&pos) {
                    acc.sum += num;
                    acc.numbers.curr.pop();
                }
            }
            acc.symbols.curr.insert(pos);
            Ok(acc)
        }
        Token::Newline => {
            acc.numbers.push_newline();
            acc.symbols.push_newline();
            Ok(acc)
        }
    }
}

/// Main part of the second problem.
///
/// Assuming any given [`GearAcc`] and [`Token`], do the following depending on the type of token:
/// - **Number**: Look for gears in the last and current line that are adjacent and push the number to them. Then remeber the number in the current line, as an upcoming gear could still utilize it.
/// - **Gear**: Look for numbers in the last and current line that are adjacent and push the number to this. Remember the gear for future numbers.
/// - **Newline**: Add all valid gears from the last line to the sum and replace it with the current line. Start with a fresh current line.
/// - **Symbol**: *ignore*
fn fold_entries_around_gears(mut acc: GearAcc, token: Result<Token>) -> Result<GearAcc> {
    match token? {
        Token::Number(range, num) => {
            for pos in range.clone() {
                if let Some(gear) = acc.gears.last.get_mut(&pos) {
                    gear.register_neighbor(num);
                }
            }
            if let Some(gear) = acc.gears.curr.get_mut(&range.start) {
                gear.register_neighbor(num);
            }
            acc.numbers.curr.push((range, num));
            Ok(acc)
        }
        Token::Gear(pos) => {
            let mut gear = Gear::default();
            for (range, num) in &acc.numbers.last {
                if range.contains(&pos) {
                    gear.register_neighbor(*num)
                }
            }
            if let Some((range, num)) = acc.numbers.curr.last() {
                if range.contains(&pos) {
                    gear.register_neighbor(*num)
                }
            }
            acc.gears.curr.insert(pos, gear);
            Ok(acc)
        }
        Token::Symbol(_) => Ok(acc),
        Token::Newline => {
            acc.sum += acc
                .gears
                .last
                .values()
                .map(|gear| gear.get_ratio())
                .sum::<usize>();
            acc.gears.push_newline();
            acc.numbers.push_newline();
            Ok(acc)
        }
    }
}

aoc_utils::main!(Day03, "inputs-03-test" => 4361, "inputs-03-test" => 467835);
