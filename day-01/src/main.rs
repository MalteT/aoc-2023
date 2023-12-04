use aoc_utils::{main, Error, Lines, Problem};

struct Day01;

impl Problem<Lines> for Day01 {
    type Solution = usize;

    fn solve_first(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        input.try_fold(0usize, |sum, line| {
            let line = line?;
            let left = line
                .chars()
                .find(|char| char.is_ascii_digit())
                .and_then(|char| char.to_digit(10))
                .ok_or_else(|| Error::input(format!("No integer on line {line:?}")))?;
            let right = line
                .chars()
                .rev()
                .find(|char| char.is_ascii_digit())
                .and_then(|char| char.to_digit(10))
                .ok_or_else(|| Error::input(format!("No integer on line {line:?}")))?;
            let pair = pair_digits((left as u8, right as u8));
            Ok(sum + pair as usize)
        })
    }

    fn solve_second(mut input: Lines) -> aoc_utils::Result<Self::Solution> {
        input.try_fold(0usize, |sum, line| {
            let line = line?;
            let left =
                State::run(&line).ok_or_else(|| Error::input(format!("a number in {line}")))?;
            let right =
                RevState::run(&line).ok_or_else(|| Error::input(format!("a number in {line}")))?;
            let pair = pair_digits((left, right));
            #[cfg(debug_assertions)]
            {
                let test = test(&line);
                if test != pair {
                    eprintln!("{test:>5} {pair:>5}");
                    eprintln!("= {pair} < {line}");
                }
            }
            Ok(sum + pair as usize)
        })
    }
}

/// Possible transitions:
///  - `efghinorstuvwx`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum State {
    #[default]
    Initial,
    O,
    On,
    T,
    Tw,
    Th,
    Thr,
    Thre,
    F,
    Fo,
    Fou,
    Fi,
    Fiv,
    S,
    Si,
    Se,
    Sev,
    Seve,
    E,
    Ei,
    Eig,
    Eigh,
    N,
    Ni,
    Nin,
}

enum StateResult<S> {
    Transition(S),
    Number(u8),
}

impl State {
    fn apply(self, byte: u8) -> StateResult<Self> {
        use State::*;
        use StateResult::*;
        match (self, byte) {
            (O, b'n') => Transition(On),
            (On, b'e') => Number(1),

            (T, b'w') => Transition(Tw),
            (Tw, b'o') => Number(2),

            (T, b'h') => Transition(Th),
            (Th, b'r') => Transition(Thr),
            (Thr, b'e') => Transition(Thre),
            (Thre, b'e') => Number(3),

            (F, b'o') => Transition(Fo),
            (Fo, b'u') => Transition(Fou),
            (Fou, b'r') => Number(4),

            (F, b'i') => Transition(Fi),
            (Fi, b'v') => Transition(Fiv),
            (Fiv, b'e') => Number(5),

            (S, b'i') => Transition(Si),
            (Si, b'x') => Number(6),

            (S, b'e') => Transition(Se),
            (Se, b'v') => Transition(Sev),
            (Sev, b'e') => Transition(Seve),
            (Seve, b'n') => Number(7),

            (E, b'i') => Transition(Ei),
            (Ei, b'g') => Transition(Eig),
            (Eig, b'h') => Transition(Eigh),
            (Eigh, b't') => Number(8),

            (N, b'i') => Transition(Ni),
            (Ni, b'n') => Transition(Nin),
            (Nin, b'e') => Number(9),

            (On, b'i') => Transition(Ni),
            (Thre, b'i') => Transition(Ei),
            (Se, b'i') => Transition(Ei),
            (Seve, b'i') => Transition(Ei),
            (Fo, b'n') => Transition(On),
            (Nin, b'i') => Transition(Ni),

            (_, b'o') => Transition(O),
            (_, b't') => Transition(T),
            (_, b'f') => Transition(F),
            (_, b's') => Transition(S),
            (_, b'e') => Transition(E),
            (_, b'n') => Transition(N),

            (_, b'1') => Number(1),
            (_, b'2') => Number(2),
            (_, b'3') => Number(3),
            (_, b'4') => Number(4),
            (_, b'5') => Number(5),
            (_, b'6') => Number(6),
            (_, b'7') => Number(7),
            (_, b'8') => Number(8),
            (_, b'9') => Number(9),

            _ => Transition(Initial),
        }
    }
    fn run<S: AsRef<str>>(input: S) -> Option<u8> {
        let mut state: Self = Default::default();
        let mut bytes = input.as_ref().bytes();
        loop {
            let byte = bytes.next()?;
            match state.apply(byte) {
                StateResult::Transition(new) => state = new,
                StateResult::Number(num) => break Some(num),
            }
        }
    }
}

#[derive(Debug, Default)]
enum RevState {
    #[default]
    Initial,
    E,
    En,
    O,
    Ow,
    Ee,
    Eer,
    Eerh,
    R,
    Ru,
    Ruo,
    Ev,
    Evi,
    X,
    Xi,
    N,
    Ne,
    Nev,
    Neve,
    T,
    Th,
    Thg,
    Thgi,
    Eni,
}

impl RevState {
    fn apply(self, byte: u8) -> StateResult<Self> {
        use RevState::*;
        use StateResult::*;
        match (self, byte) {
            (E, b'n') => Transition(En),
            (En, b'o') => Number(1),

            (O, b'w') => Transition(Ow),
            (Ow, b't') => Number(2),

            (E, b'e') => Transition(Ee),
            (Ee, b'r') => Transition(Eer),
            (Eer, b'h') => Transition(Eerh),
            (Eerh, b't') => Number(3),

            (R, b'u') => Transition(Ru),
            (Ru, b'o') => Transition(Ruo),
            (Ruo, b'f') => Number(4),

            (E, b'v') => Transition(Ev),
            (Ev, b'i') => Transition(Evi),
            (Evi, b'f') => Number(5),

            (X, b'i') => Transition(Xi),
            (Xi, b's') => Number(6),

            (N, b'e') => Transition(Ne),
            (Ne, b'v') => Transition(Nev),
            (Nev, b'e') => Transition(Neve),
            (Neve, b's') => Number(7),

            (T, b'h') => Transition(Th),
            (Th, b'g') => Transition(Thg),
            (Thg, b'i') => Transition(Thgi),
            (Thgi, b'e') => Number(8),

            (En, b'i') => Transition(Eni),
            (Eni, b'n') => Number(9),

            (Ee, b'e') => Transition(Ee),
            (Ee, b'n') => Transition(En),
            (Ee, b'v') => Transition(Ev),
            (Eer, b'u') => Transition(Ru),
            (En, b'e') => Transition(Ne),
            (Ne, b'e') => Transition(Ee),
            (Ne, b'n') => Transition(En),
            (Nev, b'i') => Transition(Evi),
            (Neve, b'e') => Transition(Ee),
            (Neve, b'n') => Transition(En),
            (Neve, b'v') => Transition(Ev),
            (Ruo, b'w') => Transition(Ow),

            (_, b'e') => Transition(E),
            (_, b'o') => Transition(O),
            (_, b'r') => Transition(R),
            (_, b'x') => Transition(X),
            (_, b'n') => Transition(N),
            (_, b't') => Transition(T),

            (_, b'1') => Number(1),
            (_, b'2') => Number(2),
            (_, b'3') => Number(3),
            (_, b'4') => Number(4),
            (_, b'5') => Number(5),
            (_, b'6') => Number(6),
            (_, b'7') => Number(7),
            (_, b'8') => Number(8),
            (_, b'9') => Number(9),

            _ => Transition(Initial),
        }
    }
    fn run<S: AsRef<str>>(input: S) -> Option<u8> {
        let mut state: Self = Default::default();
        let mut bytes = input.as_ref().bytes().rev();
        loop {
            let byte = bytes.next()?;
            match state.apply(byte) {
                StateResult::Transition(new) => state = new,
                StateResult::Number(num) => break Some(num),
            }
        }
    }
}

#[cfg(debug_assertions)]
fn test(line: &str) -> u8 {
    macro_rules! index {
        ($line:expr, $word:literal, $number:literal, $char:literal) => {
            (
                $line
                    .replace($word, &format!("{}", $number))
                    .match_indices($char)
                    .next()
                    .map(|(idx, _)| idx)
                    .unwrap_or(usize::MAX),
                $number,
            )
        };
    }
    let left = [
        index!(line, "one", 1, '1'),
        index!(line, "two", 2, '2'),
        index!(line, "three", 3, '3'),
        index!(line, "four", 4, '4'),
        index!(line, "five", 5, '5'),
        index!(line, "six", 6, '6'),
        index!(line, "seven", 7, '7'),
        index!(line, "eight", 8, '8'),
        index!(line, "nine", 9, '9'),
    ]
    .into_iter()
    .min_by_key(|(pos, _)| *pos)
    .unwrap()
    .1;

    let line = line.chars().rev().collect::<String>();
    let right = [
        index!(line, "eno", 1, '1'),
        index!(line, "owt", 2, '2'),
        index!(line, "eerht", 3, '3'),
        index!(line, "ruof", 4, '4'),
        index!(line, "evif", 5, '5'),
        index!(line, "xis", 6, '6'),
        index!(line, "neves", 7, '7'),
        index!(line, "thgie", 8, '8'),
        index!(line, "enin", 9, '9'),
    ]
    .into_iter()
    .min_by_key(|(pos, _)| *pos)
    .unwrap()
    .1;

    left as u8 * 10 + right as u8
}

fn pair_digits((left, right): (u8, u8)) -> u8 {
    left * 10 + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_inputs() {
        assert_eq!(State::run("one"), Some(1));
        assert_eq!(State::run("two"), Some(2));
        assert_eq!(State::run("three"), Some(3));
        assert_eq!(State::run("four"), Some(4));
        assert_eq!(State::run("five"), Some(5));
        assert_eq!(State::run("six"), Some(6));
        assert_eq!(State::run("seven"), Some(7));
        assert_eq!(State::run("eight"), Some(8));
        assert_eq!(State::run("nine"), Some(9));

        assert_eq!(State::run("onine"), Some(9));
        assert_eq!(State::run("threight"), Some(8));
        assert_eq!(State::run("seight"), Some(8));
        assert_eq!(State::run("seveight"), Some(8));
        assert_eq!(State::run("fone"), Some(1));
        assert_eq!(State::run("ninine"), Some(9));
    }

    #[test]
    fn stupni_elpmis() {
        assert_eq!(RevState::run("one"), Some(1));
        assert_eq!(RevState::run("two"), Some(2));
        assert_eq!(RevState::run("three"), Some(3));
        assert_eq!(RevState::run("four"), Some(4));
        assert_eq!(RevState::run("five"), Some(5));
        assert_eq!(RevState::run("six"), Some(6));
        assert_eq!(RevState::run("seven"), Some(7));
        assert_eq!(RevState::run("eight"), Some(8));
        assert_eq!(RevState::run("nine"), Some(9));

        assert_eq!(State::run("onine"), Some(9));
        assert_eq!(State::run("threight"), Some(8));
        assert_eq!(State::run("seight"), Some(8));
        assert_eq!(State::run("seveight"), Some(8));
        assert_eq!(State::run("fone"), Some(1));
        assert_eq!(State::run("ninine"), Some(9));
    }
}

main!(Day01, Lines, "inputs-01-test-first" => 142, "inputs-01-test-second" => 281);
