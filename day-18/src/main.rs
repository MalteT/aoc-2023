#![feature(iter_map_windows)]

use aoc_utils::{InputLine, Lines};

struct Day18;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Line {
    dir1: Direction,
    steps1: usize,
    dir2: Direction,
    steps2: usize,
}

impl aoc_utils::Problem<Lines<Line>> for Day18 {
    type Solution = usize;

    fn solve_first(input: Lines<Line>) -> aoc_utils::Result<Self::Solution> {
        let points = input
            .chain([Line {
                dir1: Direction::Up,
                steps1: 0,
                dir2: Direction::Up,
                steps2: 0,
            }])
            .scan((0, 0), |(curr_x, curr_y), line| {
                let pos = (*curr_x, *curr_y);
                match line.dir1 {
                    Direction::Up => {
                        *curr_y -= line.steps1 as isize;
                    }
                    Direction::Down => {
                        *curr_y += line.steps1 as isize;
                    }
                    Direction::Left => {
                        *curr_x -= line.steps1 as isize;
                    }
                    Direction::Right => {
                        *curr_x += line.steps1 as isize;
                    }
                };
                Some(pos)
            });
        Ok(polygon_area(points))
    }

    fn solve_second(input: Lines<Line>) -> aoc_utils::Result<Self::Solution> {
        let points = input
            .chain([Line {
                dir1: Direction::Up,
                steps1: 0,
                dir2: Direction::Up,
                steps2: 0,
            }])
            .scan((0, 0), |(curr_x, curr_y), line| {
                let pos = (*curr_x, *curr_y);
                match line.dir2 {
                    Direction::Up => {
                        *curr_y -= line.steps2 as isize;
                    }
                    Direction::Down => {
                        *curr_y += line.steps2 as isize;
                    }
                    Direction::Left => {
                        *curr_x -= line.steps2 as isize;
                    }
                    Direction::Right => {
                        *curr_x += line.steps2 as isize;
                    }
                };
                Some(pos)
            });
        Ok(polygon_area(points))
    }
}

pub fn polygon_area(mut pts: impl Iterator<Item = (isize, isize)>) -> usize {
    if let Some((first_x, first_y)) = pts.next() {
        let mut last = Option::<(isize, isize)>::None;
        let mut sum = 0;
        let mut outer = 0;
        for (curr_x, curr_y) in pts {
            if let Some((last_x, last_y)) = last {
                sum += cross(
                    curr_x - first_x,
                    curr_y - first_y,
                    last_x - first_x,
                    last_y - first_y,
                );
                outer += (curr_x - last_x).unsigned_abs() + (curr_y - last_y).unsigned_abs();
            } else {
                outer += (curr_x - first_x).unsigned_abs() + (curr_y - first_y).unsigned_abs();
            }
            last = Some((curr_x, curr_y))
        }
        isize::unsigned_abs(sum / 2) + outer / 2 + 1
    } else {
        0
    }
}

fn cross(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    x1 * y2 - x2 * y1
}

impl From<InputLine> for Line {
    fn from(line: InputLine) -> Self {
        let line = line.unwrap();
        let mut line = line.split_ascii_whitespace();
        let dir1 = match line.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
        };
        let steps1 = line.next().unwrap().parse().unwrap();
        let second = &line.next().unwrap();
        let second = &second[2..second.len() - 1];
        let steps2 = usize::from_str_radix(&second[0..5], 16).unwrap();
        let dir2 = match &second[5..6] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            x => unreachable!("{x:?}"),
        };
        Line {
            dir1,
            steps1,
            dir2,
            steps2,
        }
    }
}

aoc_utils::main!(Day18, "inputs-18-test" => 62, "inputs-18-test" => 952408144115);
