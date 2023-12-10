#![feature(iterator_try_collect)]
use std::{cmp::Ordering, collections::BTreeMap};

use aoc_utils::{Error, Lines, Result};

struct Day07;

type Bid = usize;
type Hand = [Card; 5];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(u8)]
enum Card {
    N2 = 2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Card {
    fn parse(hand: u8) -> Self {
        match hand {
            b'2' => Self::N2,
            b'3' => Self::N3,
            b'4' => Self::N4,
            b'5' => Self::N5,
            b'6' => Self::N6,
            b'7' => Self::N7,
            b'8' => Self::N8,
            b'9' => Self::N9,
            b'T' => Self::T,
            b'J' => Self::J,
            b'Q' => Self::Q,
            b'K' => Self::K,
            b'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

impl aoc_utils::Problem<Lines> for Day07 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let score = parse_line(input)
            .map(|res| match res {
                Ok((hand, bid)) => Ok((calculate_score(hand, None), bid)),
                Err(why) => Err(why),
            })
            .collect::<Result<BTreeMap<_, _>>>()?
            .iter()
            .enumerate()
            .fold(0_usize, |sum, (idx, (_hand, bid))| sum + bid * (idx + 1));
        Ok(score)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let score = parse_line(input)
            .map(|res| match res {
                Ok((hand, bid)) => Ok((
                    calculate_score(resolve_joker(&hand), Some(hand)),
                    (hand, bid),
                )),
                Err(why) => Err(why),
            })
            .collect::<Result<BTreeMap<_, _>>>()?
            .iter()
            .enumerate()
            .fold(0_usize, |sum, (idx, (_hand, (_, bid)))| {
                sum + bid * (idx + 1)
            });
        Ok(score)
    }
}

fn parse_hand(hand: &str) -> Hand {
    let hand = hand.as_bytes();
    [
        Card::parse(hand[0]),
        Card::parse(hand[1]),
        Card::parse(hand[2]),
        Card::parse(hand[3]),
        Card::parse(hand[4]),
    ]
}

fn parse_line(input: Lines) -> impl Iterator<Item = Result<(Hand, Bid)>> {
    input.map(|line| -> Result<(Hand, Bid)> {
        let line = line?;
        let (hand, bid) = line
            .split_once(' ')
            .ok_or_else(|| Error::input("Invalid line is missing a space"))?;
        let bid = bid.parse()?;
        let cards = parse_hand(hand);
        Ok((cards, bid))
    })
}

fn ty(cards: &[Card; 5]) -> HandType {
    let cards: BTreeMap<&Card, u8> = cards.iter().fold(BTreeMap::new(), |mut map, card| {
        *map.entry(card).or_insert(0) += 1;
        map
    });
    use HandType::*;
    let max_num_of_cards = cards.values().max().copied().unwrap_or_default();
    match max_num_of_cards {
        5 => FiveOfAKind,
        4 => FourOfAKind,
        3 => {
            if cards.values().filter(|&&count| count == 2).count() == 1 {
                FullHouse
            } else {
                ThreeOfAKind
            }
        }
        2 => {
            if cards.values().filter(|&&count| count == 2).count() >= 2 {
                TwoPair
            } else {
                OnePair
            }
        }
        1 => HighCard,
        x => unreachable!("impossible to have {x} cards"),
    }
}

fn resolve_joker(cards: &Hand) -> Hand {
    if !cards.iter().any(|card| matches!(card, Card::J)) {
        return *cards;
    }
    let card_map: BTreeMap<Card, u8> = cards.iter().fold(BTreeMap::new(), |mut map, card| {
        if map.contains_key(card) {
            *map.get_mut(card).unwrap() += 1;
            map
        } else {
            map.insert(*card, 1);
            map
        }
    });
    let (best_card, _count) = card_map
        .iter()
        .filter(|(&card, _)| card != Card::J)
        .max_by(|(left_card, left_count), (right_card, right_count)| {
            match left_count.cmp(right_count) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => left_card.cmp(right_card),
            }
        })
        // Can only be empty if every card is a joker
        // in which case the best card to choose is [`Card::A`]
        .unwrap_or((&Card::A, &5));
    let mut cards = *cards;
    for card in &mut cards {
        if *card == Card::J {
            *card = *best_card;
        }
    }
    cards
}

fn calculate_score(hand: Hand, orig: Option<Hand>) -> u32 {
    let mut score = ty(&hand) as u32;
    let orig = orig.unwrap_or(hand);
    for card in orig {
        score <<= 4;
        score |= if card == Card::J { 1 } else { card as u32 };
    }
    score
}

aoc_utils::main!(Day07, "inputs-07-test" => 6440, "inputs-07-test" => 5905);

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::{calculate_score, parse_hand, resolve_joker};

    fn cmp(left: &str, right: &str) -> Ordering {
        let left = calculate_score(resolve_joker(&parse_hand(left)), Some(parse_hand(left)));
        let right = calculate_score(resolve_joker(&parse_hand(right)), Some(parse_hand(right)));
        left.cmp(&right)
    }

    #[test]
    fn simple_comparisons() {
        assert_eq!(cmp("TJ746", "T23TA"), Ordering::Less);
        assert_eq!(cmp("AKQT2", "2TQKA"), Ordering::Greater);
        assert_eq!(cmp("22222", "22223"), Ordering::Greater);
        assert_eq!(cmp("22232", "22224"), Ordering::Greater);
        assert_eq!(cmp("222J3", "22224"), Ordering::Less);
        assert_eq!(cmp("JJJ32", "JJJ23"), Ordering::Greater);
        assert_eq!(cmp("J3232", "J2323"), Ordering::Greater);
    }
}
