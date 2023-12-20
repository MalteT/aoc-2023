use std::{
    collections::{BTreeMap, VecDeque},
    hash::{DefaultHasher, Hasher as _},
};

use aoc_utils::{Lines, RawLine};
use lazy_static::lazy_static;

type ModName = u64;

lazy_static! {
    static ref BROADCASTER: ModName = calculate_hash(&"broadcaster");
    static ref RX: ModName = calculate_hash(&"rx");
}

struct Day20;

#[derive(Debug)]
struct Network {
    modules: BTreeMap<ModName, (Module, Vec<ModName>)>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    Off,
    On,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    Broadcaster,
    FlipFlop(State),
    Conjunction(BTreeMap<ModName, Pulse>),
}

impl aoc_utils::Problem<Lines<RawLine>> for Day20 {
    type Solution = usize;

    fn solve_first(input: Lines<RawLine>) -> aoc_utils::Result<Self::Solution> {
        let input = input.map(RawLine::into_inner);
        let mut network = Network::parse(input);
        let [sum_low, sum_high] = (0..1000).map(|_| network.push_button()).fold(
            [0, 0],
            |[sum_low, sum_high], ([counter_low, counter_high], _)| {
                [sum_low + counter_low, sum_high + counter_high]
            },
        );
        Ok(sum_low * sum_high)
    }

    fn solve_second(input: Lines<RawLine>) -> aoc_utils::Result<Self::Solution> {
        let input = input.map(RawLine::into_inner);
        let mut network = Network::parse(input);
        let score = (0..)
            .take_while(|_| !network.push_button().1)
            .inspect(|num| eprintln!("{num}"))
            .count();
        Ok(score)
    }
}

impl Module {
    fn make_inputs_known(&mut self, inputs: &[ModName]) {
        match self {
            Module::Broadcaster => {}
            Module::FlipFlop(_) => {}
            Module::Conjunction(state) => inputs.iter().for_each(|module| {
                state.insert(*module, Pulse::Low);
            }),
        }
    }

    fn push(&mut self, pulse: Pulse, source: Option<ModName>) -> Option<Pulse> {
        match self {
            Module::FlipFlop(state) if pulse == Pulse::Low => match state {
                State::Off => {
                    *state = State::On;
                    Some(Pulse::High)
                }
                State::On => {
                    *state = State::Off;
                    Some(Pulse::Low)
                }
            },
            Module::Conjunction(state) => {
                state.insert(source.expect("Need a source for the conjunction"), pulse);
                if state.values().all(|inp| *inp == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Module::Broadcaster => Some(pulse),
            Module::FlipFlop(_) => None,
        }
    }
}

impl Network {
    pub fn parse(input: impl Iterator<Item = String>) -> Self {
        let mut modules: BTreeMap<u64, (Module, Vec<u64>)> = input
            .map(|line| {
                let (name, targets) = line.split_once(" -> ").unwrap();
                let (module, module_name) = if let Some(name) = name.strip_prefix('%') {
                    let module = Module::FlipFlop(State::default());
                    let name = calculate_hash(&name);
                    (module, name)
                } else if let Some(name) = name.strip_prefix('&') {
                    let module = Module::Conjunction(BTreeMap::new());
                    let name = calculate_hash(&name);
                    (module, name)
                } else if name == "broadcaster" {
                    let module = Module::Broadcaster;
                    let name = *BROADCASTER;
                    (module, name)
                } else {
                    unreachable!()
                };
                let targets = targets
                    .split(',')
                    .map(|target| calculate_hash(&target.trim()))
                    .collect();
                (module_name, (module, targets))
            })
            .collect();
        let mut rev = BTreeMap::new();
        for (module_name, (_, targets)) in &modules {
            for target in targets {
                rev.entry(*target).or_insert(vec![]).push(*module_name)
            }
        }
        for (module_name, inputs) in rev {
            if let Some((module, _)) = modules.get_mut(&module_name) {
                module.make_inputs_known(&inputs);
            }
        }
        Network { modules }
    }

    pub fn push_button(&mut self) -> ([usize; 2], bool) {
        use Pulse::*;
        let mut signals = VecDeque::new();
        let mut rx_activated = false;
        signals.push_back((*BROADCASTER, Low, None));
        let mut counter = [1, 0];
        while let Some((module_name, pulse, source)) = signals.pop_front() {
            let (module, targets) = match self.modules.get_mut(&module_name) {
                Some(module) => module,
                None => continue,
            };
            if let Some(pulse) = module.push(pulse, source) {
                targets.iter().for_each(|target| {
                    if *target == *RX && pulse == Low {
                        rx_activated = true;
                    }
                    match pulse {
                        Low => counter[0] += 1,
                        High => counter[1] += 1,
                    }
                    signals.push_back((*target, pulse, Some(module_name)));
                })
            }
        }

        (counter, rx_activated)
    }
}

fn calculate_hash<T: std::hash::Hash>(t: &T) -> ModName {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

aoc_utils::main!(Day20, "inputs-20-test" => 11687500, "inputs-20-test" => 0);
