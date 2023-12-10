use std::collections::HashMap;

use aoc_utils::Grid;

struct Day10;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    NE,
    NS,
    NW,
    EW,
    SE,
    SW,
    Blank,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Connectivity {
    Undecided,
    Circle,
    Left,
    Right,
}

impl aoc_utils::Problem<Grid<Pipe>> for Day10 {
    type Solution = usize;

    fn solve_first(grid: Grid<Pipe>) -> aoc_utils::Result<Self::Solution> {
        use Direction::*;

        let start = grid
            .find_idx(|&pipe| pipe == Pipe::Start)
            .expect("a starting position");
        #[cfg(debug_assertions)]
        eprintln!("{grid}");
        for dir in [North, East, South, West] {
            let just_count = |num, _, _| num + 1;
            let circle_len = match fold_over_circle(&grid, start, dir, 0_usize, just_count) {
                Some(circle_len) => circle_len,
                None => continue,
            };
            #[cfg(debug_assertions)]
            {
                use std::collections::HashSet;
                use yansi::Paint;
                let assemble_circle = |mut list: HashSet<_>, pos, _| {
                    list.insert(pos);
                    list
                };
                let circle =
                    fold_over_circle(&grid, start, dir, HashSet::new(), assemble_circle).unwrap();
                grid.debug_render(|pos, cell| {
                    if *cell == Pipe::Start {
                        Paint::red(format!("{cell}")).bold()
                    } else if circle.contains(&pos) {
                        Paint::new(format!("{cell}")).bold()
                    } else {
                        Paint::new(format!("{cell}")).dimmed()
                    }
                });
            }
            return Ok(circle_len / 2);
        }
        unreachable!()
    }

    fn solve_second(grid: Grid<Pipe>) -> aoc_utils::Result<Self::Solution> {
        use Connectivity::*;
        use Direction::*;

        let start = grid
            .find_idx(|&pipe| pipe == Pipe::Start)
            .expect("a starting position");
        #[cfg(debug_assertions)]
        eprintln!("{grid}");
        for dir in [North, East, South, West] {
            let accumulate_circle_map = |mut map: HashMap<_, _>, pos, dir| {
                map.insert(pos, dir);
                map
            };
            let circle =
                match fold_over_circle(&grid, start, dir, HashMap::new(), accumulate_circle_map) {
                    Some(circle) => circle,
                    None => continue,
                };
            let mut fill = grid.clone_with_fn(|pos, _cell| {
                if circle.contains_key(&pos) {
                    Circle
                } else {
                    Undecided
                }
            });
            let mut fill_queue = vec![];
            for ((x, y), dir) in &circle {
                let symbol = grid[(*x, *y)];
                let instructions = match (symbol, dir) {
                    (Pipe::NE, North) => [(West, Right), (North, Right)].as_slice(),
                    (Pipe::NE, East) => &[(South, Left), (West, Left)],
                    (Pipe::NS, North) => &[(West, Right), (East, Left)],
                    (Pipe::NS, South) => &[(West, Left), (East, Right)],
                    (Pipe::NW, North) => &[(East, Left), (South, Left)],
                    (Pipe::NW, West) => &[(South, Right), (East, Right)],
                    (Pipe::EW, East) => &[(South, Left), (North, Right)],
                    (Pipe::EW, West) => &[(North, Left), (South, Right)],
                    (Pipe::SE, East) => &[(North, Right), (West, Right)],
                    (Pipe::SE, South) => &[(North, Left), (West, Left)],
                    (Pipe::SW, South) => &[(East, Right), (North, Right)],
                    (Pipe::SW, West) => &[(East, Left), (North, Left)],
                    _ => &[],
                };
                for (dir, conn) in instructions {
                    if let Some(pos) = walk(&grid, (*x, *y), *dir) {
                        if !circle.contains_key(&pos) {
                            fill[pos] = *conn;
                            fill_queue.push(pos);
                        }
                    }
                }
            }
            while let Some(curr_pos) = fill_queue.pop() {
                [North, East, South, West]
                    .into_iter()
                    .flat_map(|dir| walk(&grid, curr_pos, dir))
                    .for_each(|next_pos| {
                        if fill[next_pos] == Undecided && !circle.contains_key(&next_pos) {
                            fill[next_pos] = fill[curr_pos];
                            fill_queue.push(next_pos);
                        }
                    });
            }
            let (outside_type, left_count, right_count) = fill.iter_pos().fold(
                (None, 0usize, 0usize),
                |(mut outside_type, mut left_count, mut right_count), ((x, y), conn)| {
                    match conn {
                        Left => left_count += 1,
                        Right => right_count += 1,
                        Undecided => panic!("Still not settled!"),
                        _ => {}
                    }
                    if (*conn == Left || *conn == Right)
                        && (x == 0 || x == fill.width() || y == 0 || y == fill.height())
                    {
                        outside_type = Some(conn);
                    }
                    (outside_type, left_count, right_count)
                },
            );
            #[cfg(debug_assertions)]
            {
                use yansi::Paint;
                fill.debug_render(|pos, cell| {
                    if grid[pos] == Pipe::Start {
                        Paint::red("S".to_owned()).bold()
                    } else if circle.contains_key(&pos) {
                        Paint::new(format!("{}", grid[pos])).bold()
                    } else if Some(cell) == outside_type {
                        Paint::new(String::from(".")).dimmed()
                    } else {
                        Paint::green(String::from("*")).bold()
                    }
                });
            }
            return match outside_type {
                Some(Left) => Ok(right_count),
                Some(Right) => Ok(left_count),
                None => panic!("Could not decide outside/inside type"),
                _ => unreachable!(),
            };
        }
        unreachable!()
    }
}

fn fold_over_circle<T, F>(
    grid: &Grid<Pipe>,
    start: (usize, usize),
    dir: Direction,
    init: T,
    acc: F,
) -> Option<T>
where
    F: Fn(T, (usize, usize), Direction) -> T,
{
    let mut last_dir = dir;
    let mut circle = init;
    let mut curr = match walk(grid, start, dir) {
        Some(pos) => pos,
        None => return None,
    };
    circle = acc(circle, curr, dir);
    while curr != start {
        let dir = grid[curr].steer(last_dir.invert())?;
        circle = acc(circle, curr, dir);
        last_dir = dir;
        curr = walk(grid, curr, dir)?;
    }
    Some(circle)
}

fn walk<T>(grid: &Grid<T>, (x, y): (usize, usize), dir: Direction) -> Option<(usize, usize)> {
    use Direction::*;
    match dir {
        North if y > 0 => Some((x, y - 1)),
        East if x + 1 < grid.width() => Some((x + 1, y)),
        South if y + 1 < grid.height() => Some((x, y + 1)),
        West if x > 0 => Some((x - 1, y)),
        _ => None,
    }
}

impl Pipe {
    fn steer(&self, old_dir: Direction) -> Option<Direction> {
        use Direction::*;
        match (self, old_dir) {
            (Pipe::NE, North) => Some(East),
            (Pipe::NE, East) => Some(North),
            (Pipe::NS, North) => Some(South),
            (Pipe::NS, South) => Some(North),
            (Pipe::NW, North) => Some(West),
            (Pipe::NW, West) => Some(North),
            (Pipe::EW, East) => Some(West),
            (Pipe::EW, West) => Some(East),
            (Pipe::SE, East) => Some(South),
            (Pipe::SE, South) => Some(East),
            (Pipe::SW, South) => Some(West),
            (Pipe::SW, West) => Some(South),
            _ => None,
        }
    }
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Pipe::NS,
            b'-' => Pipe::EW,
            b'L' => Pipe::NE,
            b'J' => Pipe::NW,
            b'7' => Pipe::SW,
            b'F' => Pipe::SE,
            b'.' => Pipe::Blank,
            b'S' => Pipe::Start,
            _ => unreachable!("{:?} is not valid here", char::from_u32(value as u32)),
        }
    }
}

impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pipe::NE => '╰',
                Pipe::NS => '│',
                Pipe::NW => '╯',
                Pipe::EW => '─',
                Pipe::SE => '╭',
                Pipe::SW => '╮',
                Pipe::Blank => '.',
                Pipe::Start => 'S',
            }
        )
    }
}

impl std::fmt::Display for Connectivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Connectivity::Undecided => '?',
                Connectivity::Left => '+',
                Connectivity::Right => '-',
                Connectivity::Circle => 'o',
            }
        )
    }
}

aoc_utils::main!(Day10, "inputs-10-test-first-2" => 8, "inputs-10-test-second" => 10);
