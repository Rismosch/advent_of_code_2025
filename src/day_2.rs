use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_2";

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse input...");
    let mut ranges = Vec::new();
    for split in input.split(',') {
        if split.is_empty() {
            continue;
        }

        let mut splits = split.trim().split('-');
        let start_str = splits.next().into_ris_error()?;
        let end_str = splits.next().into_ris_error()?;

        let start = start_str.parse::<usize>()?;
        let end = end_str.parse::<usize>()? + 1;
        let range = start..end;
        ranges.push(range);
    }

    ris_log::info!("run part 1...");
    let result = run_part_1(&ranges);
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&ranges);
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(input: &[std::ops::Range<usize>]) -> usize {
    let mut sum = 0;
    for (i, range) in input.iter().enumerate() {
        ris_log::info!(
            "run range... {}/{} {:?}",
            i + 1,
            input.len(),
            range,
        );

        for n in range.clone().into_iter() {
            if !is_valid_1(n) {
                sum += n;
            }
        }
    }

    sum
}

fn run_part_2(input: &[std::ops::Range<usize>]) -> usize {
    let mut sum = 0;
    for (i, range) in input.iter().enumerate() {
        ris_log::info!(
            "run range... {}/{} {:?}",
            i + 1,
            input.len(),
            range,
        );

        for n in range.clone().into_iter() {
            if !is_valid_2(n) {
                sum += n;
            }
        }
    }
    
    sum
}

fn is_valid_1(n: usize) -> bool {
    let n_string = n.to_string();
    if n_string.len() % 2 != 0 {
        return true;
    }

    let len = n_string.len() / 2;
    let v1 = &n_string[0..len];
    let v2 = &n_string[len..n_string.len()];

    v1 != v2
}

fn is_valid_2(n: usize) -> bool {
    let n_string = n.to_string();
    let len = n_string.len();

    // find divisors
    let mut divisors = Vec::new();
    for i in 1..len {
        if len % i == 0 {
            divisors.push(i);
        }
    }

    // test divisors
    for divisor in divisors {
        let elements = len / divisor;
        let comparisons = elements - 1;
        let mut matches = 0;
        for c in 0..comparisons {
            let i0 = c * divisor;
            let i1 = (c + 1) * divisor;
            let i2 = (c + 2) * divisor;

            let v0 = &n_string[i0..i1];
            let v1 = &n_string[i1..i2];

            if v0 == v1 {
                matches += 1;
            }
        }

        if matches == comparisons {
            return false;
        }
    }

    true
}
