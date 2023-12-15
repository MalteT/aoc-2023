use std::hash::Hasher as _;

use aoc_utils::Lines;

struct Day15;

#[derive(Debug, Default)]
struct Hasher {
    state: u64,
}

impl std::hash::Hasher for Hasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        bytes.iter().for_each(|code| {
            self.state = ((self.state + *code as u64) * 17) % 256;
        });
    }
}

#[derive(Debug)]
struct HashMap<'s> {
    boxes: [Vec<(&'s str, u8)>; u8::MAX as usize + 1],
}

enum Action<'s> {
    Remove(&'s str),
    Set(&'s str, u8),
}

impl<'s> Action<'s> {
    fn from_str(input: &'s str) -> Self {
        let marker_pos = input.find(|char| char == '=' || char == '-').unwrap();
        let (label, focal_length) = input.split_at(marker_pos);
        match input.as_bytes()[marker_pos] {
            b'=' => Action::Set(label, focal_length[1..].parse().unwrap()),
            b'-' => Action::Remove(label),
            _ => unreachable!(),
        }
    }
}

fn hash(label: &str) -> u64 {
    let mut h = Hasher::default();
    h.write(label.as_bytes());
    h.finish()
}

fn apply_action_to_hashmap<'s>(mut map: HashMap<'s>, action: Action<'s>) -> HashMap<'s> {
    match action {
        Action::Remove(label) => {
            let hash = hash(label) as usize;
            map.boxes[hash].retain(|(l, _)| *l != label);
            map
        }
        Action::Set(label, value) => {
            let mut found = false;
            let hash = hash(label) as usize;
            for (l, v) in &mut map.boxes[hash] {
                if *l == label {
                    *v = value;
                    found = true;
                    break;
                }
            }
            if !found {
                map.boxes[hash].push((label, value));
            }
            map
        }
    }
}

impl aoc_utils::Problem<Lines> for Day15 {
    type Solution = u64;

    fn solve_first(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        let line = input.next().unwrap()?;
        let value = line
            .split(',')
            .map(|entry| {
                let mut h = Hasher::default();
                h.write(entry.as_bytes());
                h.finish()
            })
            .sum();
        Ok(value)
    }

    fn solve_second(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        let line = input.next().unwrap()?;
        let map = line
            .split(',')
            .map(Action::from_str)
            .fold(HashMap::default(), apply_action_to_hashmap);

        let focal_power: usize = map
            .boxes
            .into_iter()
            .enumerate()
            .map(|(box_number, lenses)| {
                lenses
                    .into_iter()
                    .enumerate()
                    .map(move |(idx, (_lense, focal_length))| {
                        (box_number + 1) * (idx + 1) * focal_length as usize
                    })
                    .sum::<usize>()
            })
            .sum();
        Ok(focal_power as u64)
    }
}

impl<'s> Default for HashMap<'s> {
    fn default() -> Self {
        Self {
            boxes: [(); 256].map(|_| vec![]),
        }
    }
}

aoc_utils::main!(Day15, "inputs-15-test" => 1320, "inputs-15-test" => 145);
