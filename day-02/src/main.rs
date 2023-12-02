use aoc_utils::{main, Lines};

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
main!(Day02, Lines);

impl aoc_utils::Problem<Lines> for Day02 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let result = input
            .map(Result::unwrap)
            .map(Bag::parse)
            .map(Bag::minimum_possible)
            .map(Dice::power)
            .sum();
        Ok(result)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let result = input
            .map(Result::unwrap)
            .map(Bag::parse)
            .filter(Bag::is_possible)
            .map(|bag| bag.id)
            .sum();
        Ok(result)
    }
}

impl Bag {
    pub fn parse<S: AsRef<str>>(raw: S) -> Self {
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
        Self { id, sets }
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
