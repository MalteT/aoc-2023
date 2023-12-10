use aoc_utils::{main, Lines, Result};
use itertools::Itertools;

const TOTAL_DICE: Dice = Dice {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug, Default)]
struct Dice {
    red: u8,
    green: u8,
    blue: u8,
}

impl Dice {
    fn power(self) -> usize {
        let Dice { red, green, blue } = self;
        red as usize * green as usize * blue as usize
    }
}

#[derive(Debug)]
struct Bag {
    id: usize,
    sets: Vec<Dice>,
}

struct Day02;

impl aoc_utils::Problem<Lines> for Day02 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        input
            .map(Bag::parse)
            .filter_ok(Bag::is_possible)
            .map_ok(|bag| bag.id)
            .try_fold(0_usize, |sum, res| Ok(sum + res?))
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        input
            .map(Bag::parse)
            .map_ok(Bag::minimum_possible)
            .map_ok(Dice::power)
            .try_fold(0_usize, |sum, res| Ok(sum + res?))
    }
}

impl Bag {
    pub fn parse<S: AsRef<str>>(raw: std::io::Result<S>) -> Result<Self> {
        let raw = raw?;
        let raw = raw.as_ref();
        let (head, body) = raw.split_once(':').expect("a colon");
        let id = head
            .strip_prefix("Game ")
            .expect("a valid head")
            .parse()
            .expect("a valid id");
        let sets = body
            .split(';')
            .map(str::trim)
            .map(|set| {
                let mut dice = Dice::default();
                set.split(',').map(str::trim).for_each(|entry| {
                    let (number, color) = entry.split_once(' ').expect("a valid entry");
                    let number = number.parse().expect("a valid number of dice");
                    match color {
                        "red" => dice.red = number,
                        "green" => dice.green = number,
                        "blue" => dice.blue = number,
                        _ => unreachable!("there is a new color!"),
                    }
                });
                dice
            })
            .collect();
        Ok(Self { id, sets })
    }
    pub fn is_possible(&self) -> bool {
        self.sets.iter().all(|set| {
            set.red <= TOTAL_DICE.red
                && set.green <= TOTAL_DICE.green
                && set.blue <= TOTAL_DICE.blue
        })
    }
    pub fn minimum_possible(self) -> Dice {
        self.sets.iter().fold(Dice::default(), |acc, set| Dice {
            red: acc.red.max(set.red),
            green: acc.green.max(set.green),
            blue: acc.blue.max(set.blue),
        })
    }
}

main!(Day02, "inputs-02-test" => 8, "inputs-02-test" => 2286);
