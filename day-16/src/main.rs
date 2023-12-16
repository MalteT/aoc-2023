use aoc_utils::{Grid, Idx2D};

struct Day16;

#[derive(Debug)]
enum Tile {
    EmptySpace,
    MirrorNE,
    MirrorNW,
    SplitterHorizontal,
    SplitterVertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
}

impl aoc_utils::Problem<Grid<Tile>> for Day16 {
    type Solution = usize;

    fn solve_first(input: Grid<Tile>) -> aoc_utils::Result<Self::Solution> {
        let count = fill_floor((0, 0), Direction::East, &input);
        Ok(count)
    }

    fn solve_second(input: Grid<Tile>) -> aoc_utils::Result<Self::Solution> {
        let vert = (0..input.height()).flat_map(|y| {
            [
                ((0, y), Direction::East),
                ((input.width() - 1, y), Direction::West),
            ]
        });
        let horiz = (0..input.width()).flat_map(|x| {
            [
                ((x, 0), Direction::South),
                ((x, input.height() - 1), Direction::North),
            ]
        });
        let mut curr_max = 0;
        for (init, dir) in vert.chain(horiz) {
            let count = fill_floor(init, dir, &input);
            curr_max = curr_max.max(count);
        }
        Ok(curr_max)
    }
}

fn fill_floor(pos: (usize, usize), dir: Direction, grid: &Grid<Tile>) -> usize {
    use Direction as D;
    use Tile as T;
    let mut lava = grid.clone_with(0_u8);
    let mut queue = vec![(pos, dir)];
    let mut count = 0;
    macro_rules! push_if_walk {
        ($pos:expr, $dir:expr) => {
            match grid.walk($pos, $dir) {
                Some(new_pos) => queue.push((new_pos, $dir)),
                None => {}
            }
        };
    }
    while let Some((pos, dir)) = queue.pop() {
        let tile = &grid[pos];
        if lava[pos] & (dir as u8) != 0 {
            continue;
        }
        if lava[pos] == 0 {
            count += 1;
        }
        lava[pos] |= dir as u8;
        match (tile, dir) {
            (T::MirrorNE, D::North) => push_if_walk!(pos, D::West),
            (T::MirrorNE, D::East) => push_if_walk!(pos, D::South),
            (T::MirrorNE, D::South) => push_if_walk!(pos, D::East),
            (T::MirrorNE, D::West) => push_if_walk!(pos, D::North),
            (T::MirrorNW, D::North) => push_if_walk!(pos, D::East),
            (T::MirrorNW, D::East) => push_if_walk!(pos, D::North),
            (T::MirrorNW, D::South) => push_if_walk!(pos, D::West),
            (T::MirrorNW, D::West) => push_if_walk!(pos, D::South),
            (T::SplitterHorizontal, D::North | D::South) => {
                push_if_walk!(pos, D::East);
                push_if_walk!(pos, D::West);
            }
            (T::SplitterVertical, D::East | D::West) => {
                push_if_walk!(pos, D::North);
                push_if_walk!(pos, D::South);
            }
            (T::EmptySpace, _)
            | (T::SplitterHorizontal, D::East | D::West)
            | (T::SplitterVertical, D::North | D::South) => push_if_walk!(pos, dir),
        }
    }

    count
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        use Tile::*;
        match value {
            b'.' => EmptySpace,
            b'\\' => MirrorNE,
            b'/' => MirrorNW,
            b'-' => SplitterHorizontal,
            b'|' => SplitterVertical,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::EmptySpace => '.',
                Tile::MirrorNE => '\\',
                Tile::MirrorNW => '/',
                Tile::SplitterHorizontal => '-',
                Tile::SplitterVertical => '|',
            }
        )
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

aoc_utils::main!(Day16, "inputs-16-test" => 46, "inputs-16-test" => 51);
