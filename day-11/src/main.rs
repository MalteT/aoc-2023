use aoc_utils::Grid;

struct Day11;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Void,
    Galaxy,
}

impl aoc_utils::Problem<Grid<Cell>> for Day11 {
    type Solution = usize;

    fn solve_first(input: Grid<Cell>) -> aoc_utils::Result<Self::Solution> {
        let mut empty_columns = vec![true; input.width()];
        let mut empty_rows = vec![true; input.height()];
        // Collect all galaxies and calculate the empty rows and columns
        let galaxies = input
            .iter_pos()
            .filter(|(_, &cell)| cell == Cell::Galaxy)
            .fold(vec![], |mut list, ((x, y), _)| {
                empty_columns[x] = false;
                empty_rows[y] = false;
                list.push((x, y));
                list
            });
        // Debug
        #[cfg(debug_assertions)]
        {
            use yansi::Paint;
            input.debug_render(|(x, y), cell| {
                if empty_columns[x] && empty_rows[y] {
                    Paint::new("╳").dimmed().invert()
                } else if empty_columns[x] {
                    Paint::new("┈").dimmed().invert()
                } else if empty_rows[y] {
                    Paint::new("┊").dimmed().invert()
                } else if *cell == Cell::Galaxy {
                    Paint::new("@").bold().invert()
                } else {
                    Paint::new("·").dimmed().invert()
                }
            })
        }
        let mut total_distances = 0;
        for first_index in 0..galaxies.len() {
            for second_index in first_index + 1..galaxies.len() {
                let (first_x, first_y) = galaxies[first_index];
                let (second_x, second_y) = galaxies[second_index];
                // Basic distance
                let horizontal_diff = second_x.abs_diff(first_x);
                let vertical_diff = second_y.abs_diff(first_y);
                // Expansion!
                let additional_horizontal: usize = (first_x.min(second_x)..first_x.max(second_x))
                    .map(|x| empty_columns[x] as usize)
                    .sum();
                let additional_vertical: usize = (first_y.min(second_y)..first_y.max(second_y))
                    .map(|y| empty_rows[y] as usize)
                    .sum();
                total_distances +=
                    horizontal_diff + vertical_diff + additional_horizontal + additional_vertical;
            }
        }
        Ok(total_distances)
    }

    fn solve_second(input: Grid<Cell>) -> aoc_utils::Result<Self::Solution> {
        let mut empty_columns = vec![true; input.width()];
        let mut empty_rows = vec![true; input.height()];
        // Collect all galaxies and calculate the empty rows and columns
        let galaxies = input
            .iter_pos()
            .filter(|(_, &cell)| cell == Cell::Galaxy)
            .fold(vec![], |mut list, ((x, y), _)| {
                empty_columns[x] = false;
                empty_rows[y] = false;
                list.push((x, y));
                list
            });
        // Debug
        #[cfg(debug_assertions)]
        {
            use yansi::Paint;
            input.debug_render(|(x, y), cell| {
                if empty_columns[x] && empty_rows[y] {
                    Paint::new("╳").dimmed().invert()
                } else if empty_columns[x] {
                    Paint::new("┈").dimmed().invert()
                } else if empty_rows[y] {
                    Paint::new("┊").dimmed().invert()
                } else if *cell == Cell::Galaxy {
                    Paint::new("@").bold().invert()
                } else {
                    Paint::new("·").dimmed().invert()
                }
            })
        }
        let mut total_distances = 0;
        for first_index in 0..galaxies.len() {
            for second_index in first_index + 1..galaxies.len() {
                let (first_x, first_y) = galaxies[first_index];
                let (second_x, second_y) = galaxies[second_index];
                // Basic distance
                let horizontal_diff = second_x.abs_diff(first_x);
                let vertical_diff = second_y.abs_diff(first_y);
                // Expansion!
                let additional_horizontal: usize = (first_x.min(second_x)..first_x.max(second_x))
                    .map(|x| empty_columns[x] as usize * 999_999)
                    .sum();
                let additional_vertical: usize = (first_y.min(second_y)..first_y.max(second_y))
                    .map(|y| empty_rows[y] as usize * 999_999)
                    .sum();
                total_distances +=
                    horizontal_diff + vertical_diff + additional_horizontal + additional_vertical;
            }
        }
        Ok(total_distances)
    }
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Cell::Void,
            b'#' => Cell::Galaxy,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Void => '.',
                Cell::Galaxy => '#',
            }
        )
    }
}

aoc_utils::main!(Day11, "inputs-11-test" => 374, "inputs-11-test" => 82000210);
