#![feature(pattern)]
use std::collections::HashMap;

use aoc_utils::{Error, Lines, Result};
use gcd::Gcd;

struct Day08;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Directions {
    dirs: Vec<Direction>,
    next_index: usize,
}

#[derive(Debug)]
struct Nodes {
    network: Vec<(usize, usize)>,
    is_target: Vec<bool>,
}

impl Nodes {
    fn travel(&mut self, curr: usize, dir: Direction) -> usize {
        match dir {
            Direction::Left => self.network[curr].0,
            Direction::Right => self.network[curr].1,
        }
    }

    fn is_at_target(&self, curr: usize) -> bool {
        self.is_target[curr]
    }
}

impl aoc_utils::Problem<Lines> for Day08 {
    type Solution = usize;

    fn solve_first(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        let instructions = input.next().ok_or(Error::input("no header"))??;
        let instructions = parse_instructions(&instructions);
        let _ = input.next();
        let (curr, nodes) = parse_node_list(input, "AAA", "ZZZ")?;
        find_lcd_distance(curr, nodes, instructions)
    }

    fn solve_second(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        let instructions = input.next().ok_or(Error::input("no header"))??;
        let instructions = parse_instructions(&instructions);
        let _ = input.next();
        let (starters, nodes) = parse_node_list(input, "A", "Z")?;
        find_lcd_distance(starters, nodes, instructions)
    }
}

fn find_lcd_distance(
    starters: Vec<usize>,
    mut nodes: Nodes,
    mut instructions: Directions,
) -> Result<usize> {
    let mut curr = starters;
    let mut target_counts: Vec<usize> = vec![];
    let mut count = 1;

    loop {
        let inst = instructions.next().unwrap();
        curr.iter_mut().for_each(|curr| {
            *curr = nodes.travel(*curr, inst);
        });
        let mut idx = 0;
        while idx < curr.len() {
            if nodes.is_at_target(curr[idx]) {
                target_counts.push(count);
                curr.remove(idx);
            } else {
                idx += 1;
            }
        }
        if curr.is_empty() {
            break;
        } else {
            count += 1;
        }
    }

    let result = target_counts
        .into_iter()
        .reduce(|total, number| number / total.gcd_binary(number) * total)
        .unwrap();
    Ok(result)
}

fn parse_instructions(instructions: &str) -> Directions {
    let dirs = instructions
        .bytes()
        .map(|byte| match byte {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    Directions {
        dirs,
        next_index: 0,
    }
}

fn parse_node_list(
    input: Lines,
    starter_suffix: &'static str,
    target_suffix: &'static str,
) -> Result<(Vec<usize>, Nodes)> {
    let mut indices = HashMap::<_, usize>::new();
    let mut nodes = vec![];
    let mut starters = vec![];
    let mut is_target = vec![];
    for line in input {
        let line = line?;
        let (head, left, right) = parse_node_line(&line);
        indices.insert(head.to_owned(), indices.len());
        nodes.push((left.to_owned(), right.to_owned()));
        is_target.push(head.ends_with(target_suffix));
        if head.ends_with(starter_suffix) {
            starters.push(head.to_owned())
        }
    }
    let network = nodes
        .into_iter()
        .map(|(left, right)| {
            let left = *indices.get(&left).unwrap();
            let right = *indices.get(&right).unwrap();
            (left, right)
        })
        .collect();
    let starters = starters
        .into_iter()
        .map(|start| *indices.get(&start).unwrap())
        .collect();
    Ok((starters, Nodes { network, is_target }))
}

fn parse_node_line(line: &str) -> (&str, &str, &str) {
    (&line[0..3], &line[7..10], &line[12..15])
}

impl Iterator for Directions {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let elem = self.dirs[self.next_index];
        self.next_index += 1;
        self.next_index %= self.dirs.len();
        Some(elem)
    }
}

aoc_utils::main!(Day08, "inputs-08-test-first" => 6, "inputs-08-test-second" => 6);
