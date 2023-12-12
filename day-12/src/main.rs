use aoc_utils::Lines;
use cached::proc_macro::cached;
use itertools::Itertools;

struct Day12;

impl aoc_utils::Problem<Lines> for Day12 {
    type Solution = usize;

    fn solve_first(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let mut sum = 0;
        for line in input {
            let line = line?;
            sum += count_possibilities_for_line(&line)?;
        }
        Ok(sum)
    }

    fn solve_second(input: Lines) -> aoc_utils::Result<Self::Solution> {
        let mut sum = 0;
        for line in input {
            let line = line?;
            let (springs, numbers) = line.split_once(' ').unwrap();
            let line = format!(
                "{} {}",
                [springs; 5].into_iter().join("?"),
                [numbers; 5].into_iter().join(",")
            );
            sum += count_possibilities_for_line(&line)?;
        }
        Ok(sum)
    }
}

fn count_possibilities_for_line(line: &str) -> Result<usize, aoc_utils::Error> {
    let (springs, groups) = line.split_once(' ').unwrap();
    let groups = groups
        .split(',')
        .map(|raw| raw.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    let springs = springs.bytes().collect::<Vec<_>>();
    Ok(_count_possibilities_for_line(&springs, &groups))
}

#[cached(
    name = "POSSIBILITIES",
    type = "cached::UnboundCache<String, usize>",
    create = "{ cached::UnboundCache::new() }",
    convert = r#"{ format!("{springs:?}|{groups:?}") }"#
)]
fn _count_possibilities_for_line(springs: &[u8], groups: &[usize]) -> usize {
    if groups.is_empty() {
        return match springs.iter().all(|&spring| spring != b'#') {
            true => 1,
            false => 0,
        };
    }
    if Itertools::intersperse(groups.iter(), &1).sum::<usize>() > springs.len() {
        // Already to many remaining damaged springs for the rest of the line
        return 0;
    }
    match springs.first() {
        Some(b'.') => _count_possibilities_for_line(&springs[1..], groups),
        Some(b'#') => {
            if springs.len() < groups[0] {
                // Cannot skip expected x damaged springs
                return 0;
            }
            for &skip in springs.iter().take(groups[0]) {
                if skip == b'.' {
                    // The next x springs must be damaged as well
                    return 0;
                }
            }
            match springs.get(groups[0]) {
                Some(b'#') => {
                    // After the sequence of damaged springs, an operational must follow
                    0
                }
                None if groups.len() == 1 => {
                    // The damaged ones perfectly matched at the end
                    1
                }
                _ => {
                    let x = match springs.get(groups[0] + 1) {
                        Some(_) => {
                            _count_possibilities_for_line(&springs[groups[0] + 1..], &groups[1..])
                        }
                        None => 1,
                    };
                    x
                }
            }
        }
        Some(b'?') => {
            if springs.len() < groups[0] {
                // Cannot skip expected x damaged springs
                let x = _count_possibilities_for_line(&springs[1..], groups);
                return x;
            }
            for skip in 0..groups[0] {
                if springs[skip] == b'.' {
                    // The next x springs must be damaged as well
                    let x = _count_possibilities_for_line(&springs[1..], groups);
                    return x;
                }
            }
            match springs.get(groups[0]) {
                Some(b'#') => {
                    // After the sequence of damaged springs, an operational must follow

                    _count_possibilities_for_line(&springs[1..], groups)
                }
                None if groups.len() == 1 => {
                    // The damaged ones perfectly matched at the end
                    1
                }
                _ => {
                    let with_hash = match springs.get(groups[0] + 1) {
                        Some(_) => {
                            _count_possibilities_for_line(&springs[groups[0] + 1..], &groups[1..])
                        }
                        None => 1,
                    };

                    with_hash + _count_possibilities_for_line(&springs[1..], groups)
                }
            }
        }
        None => 0,
        _ => unreachable!(),
    }
}

aoc_utils::main!(Day12, "inputs-12-test" => 21, "inputs-12-test" => 525152);
