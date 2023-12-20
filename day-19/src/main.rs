use std::{
    cmp::Ordering,
    collections::BTreeMap,
    hash::{DefaultHasher, Hasher},
    ops::Range,
};

use aoc_utils::{Lines, RawLine};
use lazy_static::lazy_static;

type State = u64;

lazy_static! {
    static ref IN: State = calculate_hash(&"in");
    static ref REJECT: State = calculate_hash(&"R");
    static ref ACCEPT: State = calculate_hash(&"A");
}

struct Day19;

#[derive(Debug)]
enum Rule {
    CoolLooking(Ordering, usize),
    Musical(Ordering, usize),
    Aerodynamic(Ordering, usize),
    Shiny(Ordering, usize),
    Default,
}

#[derive(Debug, Default)]
struct Part {
    cool_looking: usize,
    musical: usize,
    aerodynamic: usize,
    shiny: usize,
}

#[derive(Debug, Clone)]
struct PartRange {
    cool_looking: Range<usize>,
    musical: Range<usize>,
    aerodynamic: Range<usize>,
    shiny: Range<usize>,
}

#[derive(Debug)]
struct Automaton {
    rules: BTreeMap<State, Vec<(Rule, State)>>,
}

impl aoc_utils::Problem<Lines<RawLine>> for Day19 {
    type Solution = usize;

    fn solve_first(input: Lines<RawLine>) -> aoc_utils::Result<Self::Solution> {
        let mut input = input.map(RawLine::into_inner);
        let automaton = Automaton::parse(input.by_ref().take_while(|line| !line.is_empty()));
        let score = input
            .map(|line| Part::parse(&line))
            .filter(|part| automaton.run(part))
            .fold(0, |sum, part| {
                sum + part.cool_looking + part.musical + part.aerodynamic + part.shiny
            });
        Ok(score)
    }

    fn solve_second(input: Lines<RawLine>) -> aoc_utils::Result<Self::Solution> {
        let mut input = input.map(RawLine::into_inner);
        let automaton = Automaton::parse(input.by_ref().take_while(|line| !line.is_empty()));
        let part = PartRange {
            cool_looking: 1..4001,
            musical: 1..4001,
            aerodynamic: 1..4001,
            shiny: 1..4001,
        };
        let score = automaton.break_up(part).into_iter().fold(0, |sum, part| {
            sum + part.cool_looking.count()
                * part.musical.count()
                * part.aerodynamic.count()
                * part.shiny.count()
        });
        Ok(score)
    }
}

impl Rule {
    fn parse(rule: &str) -> Self {
        let ineq = match rule.as_bytes()[1] {
            b'>' => Ordering::Greater,
            b'=' => Ordering::Equal,
            b'<' => Ordering::Less,
            _ => unreachable!(),
        };
        let rest = rule[2..].parse().unwrap();
        match rule.as_bytes()[0] {
            b'x' => Self::CoolLooking(ineq, rest),
            b'm' => Self::Musical(ineq, rest),
            b'a' => Self::Aerodynamic(ineq, rest),
            b's' => Self::Shiny(ineq, rest),
            _ => unreachable!(),
        }
    }

    fn check(&self, part: &Part) -> bool {
        match self {
            Rule::CoolLooking(ineq, rhs) => part.cool_looking.cmp(rhs) == *ineq,
            Rule::Musical(ineq, rhs) => part.musical.cmp(rhs) == *ineq,
            Rule::Aerodynamic(ineq, rhs) => part.aerodynamic.cmp(rhs) == *ineq,
            Rule::Shiny(ineq, rhs) => part.shiny.cmp(rhs) == *ineq,
            Rule::Default => true,
        }
    }

    fn split(&self, part: PartRange) -> (Option<PartRange>, Option<PartRange>) {
        macro_rules! splitter {
            ($attr:ident, $ineq:ident, $rhs:ident) => {{
                let ineq: &Ordering = $ineq;
                let rhs: &usize = $rhs;
                let range = part.$attr.clone();
                if range.contains(rhs) {
                    match ineq {
                        Ordering::Less => (
                            Some(PartRange {
                                $attr: range.start..*rhs,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                $attr: *rhs..range.end,
                                ..part.clone()
                            }),
                        ),
                        Ordering::Greater => (
                            Some(PartRange {
                                $attr: *rhs + 1..range.end,
                                ..part.clone()
                            }),
                            Some(PartRange {
                                $attr: range.start..*rhs + 1,
                                ..part.clone()
                            }),
                        ),
                        Ordering::Equal => unreachable!(),
                    }
                } else if range.start > *rhs {
                    // Splitter is to the left of our range
                    match ineq {
                        Ordering::Greater => (Some(part), None),
                        Ordering::Less => (None, Some(part)),
                        Ordering::Equal => unreachable!(),
                    }
                } else {
                    // Splitter is to the right of our range
                    match ineq {
                        Ordering::Greater => (None, Some(part)),
                        Ordering::Less => (Some(part), None),
                        Ordering::Equal => unreachable!(),
                    }
                }
            }};
        }
        match self {
            Rule::CoolLooking(ineq, rhs) => splitter!(cool_looking, ineq, rhs),
            Rule::Musical(ineq, rhs) => splitter!(musical, ineq, rhs),
            Rule::Aerodynamic(ineq, rhs) => splitter!(aerodynamic, ineq, rhs),
            Rule::Shiny(ineq, rhs) => splitter!(shiny, ineq, rhs),
            Rule::Default => (Some(part), None),
        }
    }
}

impl Automaton {
    fn parse(lines: impl Iterator<Item = String>) -> Automaton {
        let rules = lines
            .map(|line| {
                let (name, rules) = parse_automaton_line(&line);
                (name, rules.collect())
            })
            .collect();
        Automaton { rules }
    }
    fn run(&self, part: &Part) -> bool {
        let mut state: State = *IN;
        loop {
            if state == *ACCEPT {
                break true;
            } else if state == *REJECT {
                break false;
            }
            let rules = self.rules.get(&state).unwrap();
            for (rule, next) in rules {
                if rule.check(part) {
                    state = *next;
                    break;
                }
            }
        }
    }
    fn break_up(&self, part: PartRange) -> Vec<PartRange> {
        let mut parts = vec![(*IN, part)];
        let mut accepted = vec![];
        while let Some((state, mut part)) = parts.pop() {
            let rules = self.rules.get(&state).unwrap();
            for (rule, next) in rules {
                let (matched, rest) = rule.split(part);
                if let Some(matched) = matched {
                    if *next == *ACCEPT {
                        accepted.push(matched)
                    } else if *next != *REJECT {
                        parts.push((*next, matched));
                    }
                }
                match rest {
                    Some(rest) => part = rest,
                    None => break,
                }
            }
        }
        accepted
    }
}

impl Part {
    fn parse(line: &str) -> Self {
        line[1..line.len() - 1]
            .split(',')
            .fold(Part::default(), |part, setting| {
                let value = setting[2..].parse().unwrap();
                match setting.as_bytes()[0] {
                    b'x' => Part {
                        cool_looking: value,
                        ..part
                    },
                    b'm' => Part {
                        musical: value,
                        ..part
                    },
                    b'a' => Part {
                        aerodynamic: value,
                        ..part
                    },
                    b's' => Part {
                        shiny: value,
                        ..part
                    },
                    _ => unreachable!(),
                }
            })
    }
}

fn parse_automaton_line(line: &str) -> (State, impl Iterator<Item = (Rule, State)> + '_) {
    let (name, rest) = line.split_once('{').unwrap();
    let rules = &rest[0..rest.len() - 1];
    let name = calculate_hash(&name);
    let rules = rules.split(',').map(|rule| match rule.split_once(':') {
        Some((rule, next)) => (Rule::parse(rule), calculate_hash(&next)),
        None => (Rule::Default, calculate_hash(&rule)),
    });
    (name, rules)
}

fn calculate_hash<T: std::hash::Hash>(t: &T) -> State {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
aoc_utils::main!(Day19, "inputs-19-test" => 19114, "inputs-19-test" => 167409079868000);
