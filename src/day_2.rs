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

    let mut sum = 0;
    for (i, range) in ranges.iter().enumerate() {
        ris_log::info!(
            "run range... {}/{} {:?}",
            i + 1,
            ranges.len(),
            range,
        );

        for n in range.clone().into_iter() {
            if !is_valid(n) {
                sum += n;
            }
        }
    }

    answer.add(format!("1: {}", sum));

    Ok(())
}

fn is_valid(n: usize) -> bool {
    let n_string = n.to_string();
    if n_string.len() % 2 != 0 {
        return true;
    }

    let len = n_string.len() / 2;
    let v1 = &n_string[0..len];
    let v2 = &n_string[len..n_string.len()];

    v1 != v2
}
