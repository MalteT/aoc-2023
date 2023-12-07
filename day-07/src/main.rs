use std::{cmp::Ordering, collections::BTreeMap, str::FromStr};

use aoc_utils::{Error, Lines, Result};
use itertools::Itertools;
use num_enum::{IntoPrimitive, TryFromPrimitive};

struct Day07;

type Bid = usize;

#[derive(Debug, Eq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
enum Card {
    N2 = b'2',
    N3 = b'3',
    N4 = b'4',
    N5 = b'5',
    N6 = b'6',
    N7 = b'7',
    N8 = b'8',
    N9 = b'9',
    T = b'T',
    J = b'J',
    Q = b'Q',
    K = b'K',
    A = b'A',
}

#[derive(Debug, Eq, TryFromPrimitive, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
enum CardWithJoker {
    J = b'J',
    N2 = b'2',
    N3 = b'3',
    N4 = b'4',
    N5 = b'5',
    N6 = b'6',
    N7 = b'7',
    N8 = b'8',
    N9 = b'9',
    T = b'T',
    Q = b'Q',
    K = b'K',
    A = b'A',
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

#[derive(Debug, Eq)]
struct Hand<C: CardType> {
    cards: [C; 5],
}

#[derive(Debug, Eq)]
struct BestHand {
    best: Hand<Card>,
    base: Hand<CardWithJoker>,
}

trait DeriveType {
    fn ty(&self) -> HandType;
}

trait CardType
where
    Self: Copy + Ord + Eq + TryFromPrimitive<Primitive = u8>,
{
}
impl CardType for Card {}
impl CardType for CardWithJoker {}

impl DeriveType for Hand<Card> {
    fn ty(&self) -> HandType {
        let cards: BTreeMap<Card, u8> = self.cards.iter().fold(BTreeMap::new(), |mut map, card| {
            if map.contains_key(card) {
                *map.get_mut(card).unwrap() += 1;
                map
            } else {
                map.insert(*card, 1);
                map
            }
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
}

struct HandWithJoker {
    joker_indices: Vec<usize>,
    joker_slots: itertools::MultiProduct<std::array::IntoIter<Card, 12>>,
    cards: [Card; 5],
    empty: bool,
}

impl Iterator for HandWithJoker {
    type Item = Hand<Card>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.empty {
            return None;
        }
        if self.joker_indices.is_empty() {
            self.empty = true;
            return Some(Hand { cards: self.cards });
        }
        let mut slot = self.joker_slots.next()?;
        let mut cards = self.cards;
        for idx in &self.joker_indices {
            cards[*idx] = slot.pop().expect("Enough joker slots");
        }
        Some(Hand { cards })
    }
}

impl Hand<CardWithJoker> {
    fn explode(&self) -> HandWithJoker {
        let joker_indices: Vec<usize> = self
            .cards
            .iter()
            .enumerate()
            .filter(|(_, &card)| card == CardWithJoker::J)
            .map(|(idx, _)| idx)
            .collect();
        use Card::*;
        let joker_slots = (0..joker_indices.len())
            .map(|_| [N2, N3, N4, N5, N6, N7, N8, N9, T, Q, K, A])
            .multi_cartesian_product();
        macro_rules! card {
            ($idx:literal) => {
                if self.cards[$idx] == CardWithJoker::J {
                    Card::N2
                } else {
                    let prim: u8 = self.cards[$idx].into();
                    prim.try_into().unwrap()
                }
            };
        }
        HandWithJoker {
            joker_indices,
            joker_slots,
            cards: [card!(0), card!(1), card!(2), card!(3), card!(4)],
            empty: false,
        }
    }
}

impl aoc_utils::Problem<Lines> for Day07 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let hands: BTreeMap<Hand<Card>, Bid> = parse_line(input).collect::<Result<_>>()?;
        let score = hands
            .iter()
            .enumerate()
            .fold(0_usize, |sum, (idx, (_hand, bid))| sum + bid * (idx + 1));
        Ok(score)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let hands: BTreeMap<BestHand, Bid> = parse_line::<Hand<CardWithJoker>>(input)
            .map(|hand| match hand {
                Ok((hand, bid)) => Ok((
                    BestHand {
                        best: hand.explode().max().unwrap(),
                        base: hand,
                    },
                    bid,
                )),
                Err(why) => Err(why),
            })
            .collect::<Result<_>>()?;
        let score = hands
            .iter()
            .enumerate()
            .fold(0_usize, |sum, (idx, (_hand, bid))| sum + bid * (idx + 1));
        Ok(score)
    }
}

fn parse_line<H: FromStr<Err = Error>>(input: Lines) -> impl Iterator<Item = Result<(H, Bid)>> {
    input.map(|line| {
        let line = line?;
        let (hand, bid) = line
            .split_once(' ')
            .ok_or_else(|| Error::input("Invalid line is missing a space"))?;
        Ok((hand.parse()?, bid.parse()?))
    })
}

impl<C: CardType> FromStr for Hand<C> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut cards_iter = s.bytes().map(C::try_from_primitive);
        macro_rules! next {
            ($it:expr) => {
                $it.next()
                    .ok_or_else(|| Error::input("card list too short"))?
                    .map_err(|_| Error::input("invalid card found"))?
            };
        }
        let cards = [
            next!(cards_iter),
            next!(cards_iter),
            next!(cards_iter),
            next!(cards_iter),
            next!(cards_iter),
        ];

        Ok(Self { cards })
    }
}

impl<C: CardType> PartialEq for Hand<C> {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl<C: CardType> PartialOrd for Hand<C>
where
    Hand<C>: DeriveType,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: CardType> Ord for Hand<C>
where
    Hand<C>: DeriveType,
{
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        match self.ty().cmp(&other.ty()) {
            Less => Less,
            Greater => Greater,
            Equal => {
                let mut idx = 0;
                loop {
                    match self.cards[idx].cmp(&other.cards[idx]) {
                        Less => break Less,
                        Greater => break Greater,
                        Equal if idx == self.cards.len() - 1 => break Equal,
                        Equal => {}
                    }
                    idx += 1;
                }
            }
        }
    }
}

impl PartialEq for BestHand {
    fn eq(&self, other: &Self) -> bool {
        self.base.eq(&other.base)
    }
}

impl PartialOrd for BestHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BestHand {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        match self.best.ty().cmp(&other.best.ty()) {
            Less => Less,
            Greater => Greater,
            Equal => {
                let mut idx = 0;
                loop {
                    match self.base.cards[idx].cmp(&other.base.cards[idx]) {
                        Less => break Less,
                        Greater => break Greater,
                        Equal if idx == self.base.cards.len() - 1 => break Equal,
                        Equal => {}
                    }
                    idx += 1;
                }
            }
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        use Card::*;
        matches!(
            (self, other),
            (N2, N2)
                | (N3, N3)
                | (N4, N4)
                | (N5, N5)
                | (N6, N6)
                | (N7, N7)
                | (N8, N8)
                | (N9, N9)
                | (T, T)
                | (J, J)
                | (Q, Q)
                | (K, K)
                | (A, A),
        )
    }
}

impl PartialEq for CardWithJoker {
    fn eq(&self, other: &Self) -> bool {
        use CardWithJoker::*;
        matches!(
            (self, other),
            (J, J)
                | (N2, N2)
                | (N3, N3)
                | (N4, N4)
                | (N5, N5)
                | (N6, N6)
                | (N7, N7)
                | (N8, N8)
                | (N9, N9)
                | (T, T)
                | (Q, Q)
                | (K, K)
                | (A, A),
        )
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        use Card::*;
        use Ordering::*;
        let numeric = [N2, N3, N4, N5, N6, N7, N8, N9];
        if numeric.contains(self) {
            if numeric.contains(other) {
                let this: u8 = (*self).into();
                let other = (*other).into();
                this.cmp(&other)
            } else {
                Less
            }
        } else if numeric.contains(other) {
            Greater
        } else if self == other {
            Equal
        } else {
            match (self, other) {
                (A, _) | (K, Q | J | T) | (Q, J | T) | (J, T) => Greater,
                _ => Less,
            }
        }
    }
}

impl PartialOrd for CardWithJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardWithJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        use CardWithJoker::*;
        use Ordering::*;
        let numeric = [N2, N3, N4, N5, N6, N7, N8, N9];
        if numeric.contains(self) {
            if numeric.contains(other) {
                let this: u8 = (*self).into();
                let other = (*other).into();
                this.cmp(&other)
            } else if *other == J {
                Greater
            } else {
                Less
            }
        } else if numeric.contains(other) {
            if *self == J {
                Less
            } else {
                Greater
            }
        } else if self == other {
            Equal
        } else {
            match (self, other) {
                (A, _) | (K, Q | T | J) | (Q, T | J) | (T, J) => Greater,
                _ => Less,
            }
        }
    }
}

aoc_utils::main!(Day07, Lines, "inputs-07-test" => 6440, "inputs-07-test" => 5905);
