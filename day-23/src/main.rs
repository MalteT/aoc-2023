use aoc_utils::{Grid, Idx2D};

type Set<T> = std::collections::BTreeSet<T>;
type Map<A, B> = std::collections::BTreeMap<A, B>;

const ALL_DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

struct Day23;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn reverse(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => West,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Plot {
    Path,
    Forest,
    Slope(Direction),
}

struct Graph {
    nodes: Vec<Idx2D>,
    // Node indices -> (Node indice, weight)[]
    neighbors: Map<usize, Set<(usize, usize)>>,
    directed: bool,
}

impl aoc_utils::Problem<Grid<Plot>> for Day23 {
    type Solution = usize;

    fn solve_first(input: Grid<Plot>) -> aoc_utils::Result<Self::Solution> {
        let score = Graph::from(&input, true).bellman_ford_len(&input);
        Ok(score)
    }

    fn solve_second(input: Grid<Plot>) -> aoc_utils::Result<Self::Solution> {
        let score = Graph::from(&input, false).longest_path_len(&input);
        Ok(score)
    }
}

fn neighbors_no_forest<'g>(
    grid: &'g Grid<Plot>,
    pos: Idx2D,
    dirs: &'g [Direction],
) -> impl Iterator<Item = Idx2D> + 'g {
    dirs.iter()
        .flat_map(move |dir| grid.walk(pos, *dir))
        .filter(|neighbor| grid[*neighbor] != Plot::Forest)
}

fn neighbors_no_forest_star_but_count_steps(
    grid: &Grid<Plot>,
    pos: Idx2D,
    dir: Direction,
    reverse_dirs: bool,
    directed: bool,
) -> Option<(Idx2D, usize)> {
    let mut prev = pos;
    if directed {
        // Check whether the direction matches any slope that may be present
        if let Plot::Slope(req) = grid[prev] {
            if (!reverse_dirs && req != dir) || (reverse_dirs && req.reverse() != dir) {
                return None;
            }
        }
    }
    let mut curr = grid.walk(pos, dir)?;
    let mut dist = 0;
    loop {
        let (next, other) = match (directed, grid[curr]) {
            (true, Plot::Slope(dir)) => {
                let dirs = if reverse_dirs { [dir.reverse()] } else { [dir] };
                let mut iter = neighbors_no_forest(grid, curr, &dirs).filter(|next| *next != prev);
                (iter.next(), iter.next())
            }
            (_, Plot::Forest) => break None,
            _ => {
                let mut iter =
                    neighbors_no_forest(grid, curr, ALL_DIRECTIONS).filter(|next| *next != prev);
                (iter.next(), iter.next())
            }
        };
        match next {
            // Dead end
            None => break None,
            // Only one next
            Some(next) if other.is_none() => {
                prev = curr;
                curr = next;
                dist += 1;
            }
            // Multiple next
            _ => break Some((curr, dist + 1)),
        }
    }
}

impl Graph {
    pub fn longest_path_len(&self, grid: &Grid<Plot>) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        let start = grid
            .row(0)
            .find(|(_, plot)| **plot != Plot::Forest)
            .unwrap()
            .0;
        let (start_in_network, first_steps) = neighbors_no_forest_star_but_count_steps(
            grid,
            start,
            Direction::South,
            false,
            self.directed,
        )
        .unwrap();
        let start_in_network_idx = self.to_index(start_in_network);
        let target = grid
            .row(grid.height() - 1)
            .find(|(_, plot)| **plot != Plot::Forest)
            .unwrap()
            .0;
        let (target_in_network, final_steps) = neighbors_no_forest_star_but_count_steps(
            grid,
            target,
            Direction::North,
            true,
            self.directed,
        )
        .unwrap();
        let target_in_network_idx = self.to_index(target_in_network);
        first_steps
            + _longest_path_len(
                self,
                start_in_network_idx,
                target_in_network_idx,
                Set::new(),
            )
            + final_steps
    }

    fn bellman_ford_len(&self, grid: &Grid<Plot>) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
        eprintln!("{self:#?}");
        let (start, first_steps, end, final_steps) = self.find_start_and_end_indices(grid);
        let mut distances = vec![0; self.nodes.len()];
        let mut predecessor = vec![None; self.nodes.len()];
        eprintln!("{start} -> {end}");

        for _ in 0..self.nodes.len() - 1 {
            eprintln!("{distances:?}");
            for (source, targets) in &self.neighbors {
                for (target, dist) in targets {
                    if distances[*target] < distances[*source] + *dist
                        && predecessor[*source] != Some(*target)
                    {
                        eprintln!(" o> distances[*target = {target}] = distances[*source = {source}] + *dist = {}" ,distances[*source] + *dist);
                        distances[*target] = distances[*source] + *dist;
                        predecessor[*target] = Some(*source);
                    }
                }
            }
        }
        first_steps + distances[end] + final_steps
    }

    fn to_index(&self, node: Idx2D) -> usize {
        self.nodes
            .iter()
            .enumerate()
            .find(|(_, p)| **p == node)
            .unwrap()
            .0
    }

    fn find_start_and_end_indices(&self, grid: &Grid<Plot>) -> (usize, usize, usize, usize) {
        let start = grid
            .row(0)
            .find(|(_, plot)| **plot != Plot::Forest)
            .unwrap()
            .0;
        let (start_in_network, first_steps) = neighbors_no_forest_star_but_count_steps(
            grid,
            start,
            Direction::South,
            false,
            self.directed,
        )
        .unwrap();
        let start_in_network_idx = self.to_index(start_in_network);
        let target = grid
            .row(grid.height() - 1)
            .find(|(_, plot)| **plot != Plot::Forest)
            .unwrap()
            .0;
        let (target_in_network, final_steps) = neighbors_no_forest_star_but_count_steps(
            grid,
            target,
            Direction::North,
            true,
            self.directed,
        )
        .unwrap();
        let target_in_network_idx = self.to_index(target_in_network);
        (
            start_in_network_idx,
            first_steps,
            target_in_network_idx,
            final_steps,
        )
    }

    fn from(grid: &Grid<Plot>, directed: bool) -> Self {
        let nodes = grid
            .iter_pos()
            .filter(|(_, plot)| **plot != Plot::Forest)
            .filter(|(pos, _)| neighbors_no_forest(grid, *pos, ALL_DIRECTIONS).count() > 2)
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();
        let mut neighbors = Map::new();

        for idx in 0..nodes.len() {
            let start_node = nodes[idx];
            let neighbor_indicess = ALL_DIRECTIONS
                .iter()
                .flat_map(|dir| {
                    neighbors_no_forest_star_but_count_steps(
                        grid, start_node, *dir, false, directed,
                    )
                })
                .map(|(pos, dist)| {
                    let idx = nodes
                        .iter()
                        .enumerate()
                        .find(|(_, p)| **p == pos)
                        .unwrap()
                        .0;
                    (idx, dist)
                });
            for (neighbor, dist) in neighbor_indicess {
                neighbors
                    .entry(idx)
                    .or_insert_with(Set::new)
                    .insert((neighbor, dist));
            }
        }

        Self {
            nodes,
            neighbors,
            directed,
        }
    }
}

#[cached::proc_macro::cached(
    type = "cached::UnboundCache<u64, usize>",
    create = "{ cached::UnboundCache::new() }",
    convert = r#"{
        use std::hash::Hasher as _;
        let mut hash = std::collections::hash_map::DefaultHasher::new();
        hash.write_usize(start);
        hash.write_usize(target);
        seen.iter().for_each(|seen| hash.write_usize(*seen));
        hash.finish()
    }"#
)]
fn _longest_path_len(graph: &Graph, start: usize, target: usize, mut seen: Set<usize>) -> usize {
    if start == target {
        0
    } else {
        seen.insert(start);
        graph
            .neighbors
            .get(&start)
            .unwrap()
            .iter()
            .filter(|(pos, _dist)| !seen.contains(pos))
            .map(|(pos, dist)| _longest_path_len(graph, *pos, target, seen.clone()) + dist)
            .max()
            .unwrap_or(0)
    }
}

impl From<u8> for Plot {
    fn from(value: u8) -> Self {
        use Direction::*;
        use Plot::*;
        match value {
            b'.' => Path,
            b'#' => Forest,
            b'^' => Slope(North),
            b'>' => Slope(East),
            b'v' => Slope(South),
            b'<' => Slope(West),
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Debug for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Plot::Path => write!(f, "."),
            Plot::Forest => write!(f, "#"),
            Plot::Slope(dir) => write!(f, "{:?}", dir),
        }
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
            }
        )
    }
}

impl From<Direction> for (isize, isize) {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let neighbors: Vec<_> = self
            .neighbors
            .iter()
            .flat_map(|(from, tos)| {
                tos.iter()
                    .map(move |(to, dist)| format!("{:>2} -{:2>}-> {:>2}", from, dist, to))
            })
            .collect();
        f.debug_struct("Graph")
            .field("nodes", &self.nodes)
            .field("neighbors", &neighbors)
            .finish()
    }
}

aoc_utils::main!(Day23, "inputs-23-test" => 94, "inputs-23-test" => 154);
