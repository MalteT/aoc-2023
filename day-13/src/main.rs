#![feature(iter_map_windows)]
use aoc_utils::Lines;

struct DayXX;

impl aoc_utils::Problem<Lines> for DayXX {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let mut sum = 0;
        let mut curr_block: Vec<Vec<u8>> = vec![];
        for line in input.chain(vec![Ok(String::new())]) {
            let line: Vec<_> = line?.bytes().collect();
            if line.is_empty() {
                sum += find_axis(&curr_block);
                curr_block = vec![];
            } else {
                curr_block.push(line);
            }
        }
        Ok(sum)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let mut sum = 0;
        let mut curr_block: Vec<Vec<u8>> = vec![];
        for line in input.chain(vec![Ok(String::new())]) {
            let line: Vec<_> = line?.bytes().collect();
            if line.is_empty() {
                let mut smudge_pos = 0;
                let horiz_orig = find_horizontal_reflection_axis(&curr_block, None);
                let vert_orig = find_vertical_reflection_axis(&curr_block, None);
                sum += loop {
                    if smudge_pos >= curr_block.len() * curr_block[0].len() {
                        panic!("Index outside..")
                    }
                    let smudge_x = smudge_pos % curr_block[0].len();
                    let smudge_y = smudge_pos / curr_block[0].len();
                    let smudged = curr_block[smudge_y][smudge_x];
                    curr_block[smudge_y][smudge_x] = match smudged {
                        b'#' => b'.',
                        b'.' => b'#',
                        _ => unreachable!(),
                    };
                    let horiz = find_horizontal_reflection_axis(&curr_block, horiz_orig);
                    let vert = find_vertical_reflection_axis(&curr_block, vert_orig);
                    if vert.is_some() || horiz.is_some() {
                        let horiz_points = horiz.unwrap_or_default();
                        let vert_points = vert.unwrap_or_default() * 100;
                        break horiz_points + vert_points;
                    }
                    // Reset block
                    curr_block[smudge_y][smudge_x] = smudged;
                    smudge_pos += 1;
                };
                curr_block = vec![];
            } else {
                curr_block.push(line);
            }
        }
        Ok(sum)
    }
}

pub fn find_axis(block: &[Vec<u8>]) -> usize {
    let mut sum = 0;
    if let Some(pos) = find_vertical_reflection_axis(block, None) {
        sum += pos * 100
    }
    if let Some(pos) = find_horizontal_reflection_axis(block, None) {
        sum += pos
    }
    sum
}

fn find_vertical_reflection_axis(block: &[Vec<u8>], ignore: Option<usize>) -> Option<usize> {
    let vertical = block
        .iter()
        .map_windows(|[upper, lower]| *upper == *lower)
        .enumerate()
        .filter(|(_, equal)| *equal)
        .map(|(pos, _)| pos);
    for option in vertical {
        if Some(option + 1) == ignore {
            continue;
        }
        let mut offset = 0;
        let is_match = loop {
            if offset > option || option + offset + 1 == block.len() {
                break true;
            }
            if block[option - offset] != block[option + offset + 1] {
                break false;
            }
            offset += 1;
        };
        if is_match {
            return Some(option + 1);
        }
    }
    None
}

fn find_horizontal_reflection_axis(block: &[Vec<u8>], ignore: Option<usize>) -> Option<usize> {
    let horizontal = block[0]
        .iter()
        .map_windows(|[left, right]| *left == *right)
        .enumerate()
        .filter(|(_, equal)| *equal)
        .map(|(pos, _)| pos);
    for option in horizontal {
        if Some(option + 1) == ignore {
            continue;
        }
        let mut offset = 0;
        let is_match = 'main: loop {
            if offset > option || option + offset + 1 == block[0].len() {
                break true;
            }
            for line in block {
                if line[option - offset] != line[option + offset + 1] {
                    break 'main false;
                }
            }
            offset += 1;
        };
        if is_match {
            return Some(option + 1);
        }
    }
    None
}

aoc_utils::main!(DayXX, "inputs-13-test" => 405, "inputs-13-test" => 400);
