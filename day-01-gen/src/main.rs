#![feature(unix_sigpipe)]
use std::{io::Write, num::NonZeroUsize};

use clap::Parser;
use rand::{rngs::StdRng, seq::SliceRandom, thread_rng, Rng, SeedableRng};

#[derive(Debug, Parser)]
struct Args {
    #[arg(short = 'n', long, default_value_t = 1000)]
    lines: usize,
    #[arg(short = 'm', long, default_value_t = NonZeroUsize::new(1).unwrap())]
    min_line: NonZeroUsize,
    #[arg(short = 'x', long, default_value_t = 100)]
    max_line: usize,
    #[arg(long, default_value_t = 0.1)]
    numbers_per_char: f32,
    #[arg(long, short)]
    seed: Option<u64>,
}

#[unix_sigpipe = "sig_dfl"]
fn main() {
    let args = Args::parse();
    let seed = args.seed.unwrap_or_else(|| thread_rng().gen());
    let mut rng = StdRng::seed_from_u64(seed);
    let mut stdout = std::io::stdout().lock();

    (0..args.lines).for_each(|_| {
        let line = generate_line(&mut rng, &args);
        if let Err(why) = writeln!(stdout, "{line}") {
            eprintln!("{why}");
        }
    });

    eprintln!("Replicate with:");
    eprintln!("---");
    eprintln!(
        "day-01-gen --lines {} --min-line {} --max-line {} --numbers_per_char {} --seed {}",
        args.lines, args.min_line, args.max_line, args.numbers_per_char, seed
    );
    eprintln!("---");
}

fn generate_line<R: Rng>(rng: &mut R, args: &Args) -> String {
    let min = args.min_line.get();
    let max = args.max_line;
    let length = rng.gen_range(min..=max);
    // Generate just lower case letters at first
    let mut result: String = (0..length)
        .map(|_| {
            let byte = rng.gen_range(b'a'..=b'z');
            byte as char
        })
        .collect();
    let numbers = ((length as f32 * args.numbers_per_char) as usize).max(1);
    let numbers_available = numbers_for_length(length);
    let selection = numbers_available.choose_multiple(rng, numbers);
    for number in selection {
        let max_index = length - number.len();
        let pos = rng.gen_range(0..=max_index);
        result.replace_range(pos..pos + number.len(), number);
    }
    result
}

fn numbers_for_length(max_length: usize) -> Vec<&'static str> {
    [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ]
    .into_iter()
    .filter(|option| option.len() <= max_length)
    .collect()
}
