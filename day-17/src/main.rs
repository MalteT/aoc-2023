use std::{cmp::Reverse, collections::HashMap, num::NonZeroU8};

use aoc_utils::{Grid, Idx2D};
use priority_queue::PriorityQueue;
use yansi::{Color, Paint};

struct Day17;

struct Number(NonZeroU8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl aoc_utils::Problem<Grid<Number>> for Day17 {
    type Solution = usize;

    fn solve_first(input: Grid<Number>) -> aoc_utils::Result<Self::Solution> {
        let path = a_star_3(&input, (0, 0), (input.width() - 1, input.height() - 1));
        let score = path.iter().map(|pos| input[*pos].get() as usize).sum();
        input.debug_render(|pos, val| {
            let color = colorous::BLUES.eval_rational(val.get() as usize, 9);
            if path.iter().any(|p| *p == pos) || pos == (0, 0) {
                Paint::new(val.get())
                    .fg(yansi::Color::RGB(color.r, color.g, color.b))
                    .bg(Color::RGB(150, 0, 0))
            } else {
                Paint::new(val.get())
                    .fg(yansi::Color::RGB(color.r, color.g, color.b))
                    .bg(Color::RGB(0, 0, 0))
            }
        });
        Ok(score)
    }

    fn solve_second(_input: Grid<Number>) -> aoc_utils::Result<Self::Solution> {
        todo!()
    }
}

fn a_star_3(grid: &Grid<Number>, start: Idx2D, goal: Idx2D) -> Vec<Idx2D> {
    use Direction as D;
    let mut open_set = PriorityQueue::new();
    let mut best_known = HashMap::new();
    let mut previous = HashMap::new();
    let possible_neighbors = [D::North, D::East, D::South, D::West];

    let heuristic = |(x, y): Idx2D| {
        let (goal_x, goal_y) = goal;
        let x_diff = goal_x.abs_diff(x);
        let y_diff = goal_y.abs_diff(y);
        x_diff + y_diff
    };

    best_known.insert(start, 0_usize);
    open_set.push((start, D::North), Reverse(heuristic(start)));

    while let Some(((curr, last_dir), score)) = open_set.pop() {
        if curr == goal {
            let mut path = vec![];
            let mut curr = curr;
            while curr != start {
                path.push(curr);
                curr = *previous.get(&curr).unwrap();
            }
            return path;
        }
        eprint!("{curr:>2?} ({:>3}) => ", score.0);
        possible_neighbors
            .iter()
            .filter(|dir| **dir != last_dir)
            .for_each(|dir| {
                let mut prev = curr;
                for _step in 0..3 {
                    let next = grid.walk(prev, *dir);
                    if let Some(next) = next {
                        eprint!("{next:?},");
                        let cost_of_next = grid[next].get() as usize;
                        let tentative_score = best_known
                            .get(&prev)
                            .unwrap_or(&usize::MAX)
                            .saturating_add(cost_of_next);
                        if tentative_score < *best_known.get(&next).unwrap_or(&usize::MAX) {
                            previous.insert(next, prev);
                            best_known.insert(next, tentative_score);
                            open_set.push((next, *dir), Reverse(tentative_score + heuristic(next)));
                        }
                        prev = next;
                    } else {
                        break;
                    }
                }
            });
        eprintln!();
    }
    unreachable!()
}

impl TryFrom<u8> for Number {
    type Error = aoc_utils::Error;

    fn try_from(raw: u8) -> Result<Self, Self::Error> {
        match raw {
            b'1'..=b'9' => Ok(Number(NonZeroU8::new(raw - b'0').unwrap())),
            _ => Err(aoc_utils::Error::input("Invalid number in grid")),
        }
    }
}

impl std::ops::Deref for Number {
    type Target = NonZeroU8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Direction> for Idx2D<isize> {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

aoc_utils::main!(Day17, "inputs-xx" => 0, "inputs-xx" => 0);
