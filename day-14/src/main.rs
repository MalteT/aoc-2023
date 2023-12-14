use std::collections::HashMap;

use aoc_utils::Lines;

struct DayXX;

impl aoc_utils::Problem<Lines> for DayXX {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let mut lines = input
            .map(|res| match res {
                Ok(line) => Ok(line.bytes().collect::<Vec<_>>()),
                Err(why) => Err(why),
            })
            .collect::<::std::io::Result<Vec<_>>>()?;
        tilt_north(&mut lines);
        Ok(calculate_score(&lines))
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let mut lines = input
            .map(|res| match res {
                Ok(line) => Ok(line.bytes().collect::<Vec<_>>()),
                Err(why) => Err(why),
            })
            .collect::<::std::io::Result<Vec<_>>>()?;
        let mut seen = HashMap::new();
        let mut found = false;
        let mut curr_iteration = 0;
        let total_iterations = 1_000_000_000;
        while curr_iteration < total_iterations {
            cycle(&mut lines);
            if !found {
                if seen.contains_key(&lines) {
                    let same_at = seen.get(&lines).unwrap();
                    let step_size = curr_iteration - same_at;
                    let simulations_todo = total_iterations - curr_iteration;
                    let todo_divisible_by = simulations_todo / step_size;
                    let skip = todo_divisible_by * step_size;
                    curr_iteration += skip;
                    found = true;
                }
                seen.insert(lines.clone(), curr_iteration);
            }
            curr_iteration += 1;
        }

        Ok(calculate_score(&lines))
    }
}

fn calculate_score(lines: &[Vec<u8>]) -> usize {
    let mut total = 0;
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            match lines[y][x] {
                b'O' => {
                    total += lines.len() - y;
                }
                b'#' => {}
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
    total
}

fn cycle(lines: &mut [Vec<u8>]) {
    tilt_north(lines);
    tilt_west(lines);
    tilt_south(lines);
    tilt_east(lines);
}

fn tilt_north(lines: &mut [Vec<u8>]) {
    let mut next_free_pos = vec![0; lines[0].len()];
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            match lines[y][x] {
                b'O' => {
                    lines[y][x] = b'.';
                    lines[next_free_pos[x]][x] = b'O';
                    next_free_pos[x] += 1;
                }
                b'#' => {
                    next_free_pos[x] = y + 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn tilt_west(lines: &mut [Vec<u8>]) {
    for line in lines {
        let mut next_free_pos = 0;
        for x in 0..line.len() {
            match line[x] {
                b'O' => {
                    line[x] = b'.';
                    line[next_free_pos] = b'O';
                    next_free_pos += 1;
                }
                b'#' => {
                    next_free_pos = x + 1;
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn tilt_south(lines: &mut [Vec<u8>]) {
    let mut next_free_pos = vec![lines.len() - 1; lines[0].len()];
    for y in 0..lines.len() {
        let y = lines.len() - 1 - y;
        for x in 0..lines[y].len() {
            match lines[y][x] {
                b'O' => {
                    lines[y][x] = b'.';
                    lines[next_free_pos[x]][x] = b'O';
                    next_free_pos[x] -= 1;
                }
                b'#' => {
                    next_free_pos[x] = y.saturating_sub(1);
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn tilt_east(lines: &mut [Vec<u8>]) {
    for line in lines {
        let mut next_free_pos = line.len() - 1;
        for x in 0..line.len() {
            let x = line.len() - 1 - x;
            match line[x] {
                b'O' => {
                    line[x] = b'.';
                    line[next_free_pos] = b'O';
                    next_free_pos -= 1;
                }
                b'#' => {
                    next_free_pos = x.saturating_sub(1);
                }
                b'.' => {}
                _ => unreachable!(),
            }
        }
    }
}

aoc_utils::main!(DayXX, "inputs-14-test" => 136, "inputs-14-test" => 64);
