use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_5";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct IdRange {
    min: usize,
    max: usize,
}

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse ranges...");
    let mut lines = input.lines();

    let mut id_ranges = Vec::new();
    for line in lines.by_ref().take_while(|x| !x.is_empty()) {
        let splits = line.split('-').collect::<Vec<_>>();
        let min_str = splits.get(0).into_ris_error()?;
        let max_str = splits.get(1).into_ris_error()?;
        let min = min_str.parse()?;
        let max = max_str.parse()?;
        let id_range = IdRange {
            min,
            max,
        };
        id_ranges.push(Some(id_range));
    }

    ris_log::info!("parse ids...");
    let mut ids = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let id = line.trim().parse::<usize>()?;
        ids.push(id);
    }

    ris_log::info!("sort ranges...");
    id_ranges.sort_by(|lhs, rhs| {
        let lhs = lhs.expect("no None element to be present");
        let rhs = rhs.expect("no None element to be present");
        lhs.min.cmp(&rhs.min)
    });

    ris_log::info!("sort ids...");
    ids.sort();

    ris_log::trace!("ranges: {:#?}", id_ranges);

    ris_log::info!("merge ranges...");
    let mut merged_id_ranges = Vec::new();
    let mut i = 0;
    loop {
        if i >= id_ranges.len() {
            break;
        }

        let Some(mut merged_id_range) = id_ranges[i].take() else {
            i += 1;
            continue;
        };

        for candidate in id_ranges.iter_mut().skip(i + 1) {
            let Some(id_range) = candidate.clone() else {
                break;
            };

            if merged_id_range.max >= id_range.min {
                let max = usize::max(merged_id_range.max, id_range.max);
                merged_id_range.max = max;
                *candidate = None;
            }
        }

        merged_id_ranges.push(merged_id_range);
    }

    ris_log::trace!("merged ranges: {:#?}", merged_id_ranges);

    ris_log::info!("run part 1...");
    let result = run_part_1(&merged_id_ranges, &ids)?;
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&merged_id_ranges, &ids);
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(id_ranges: &[IdRange], ids: &[usize]) -> RisResult<usize> {
    let mut sum = 0;

    let mut i = 0;
    let mut j = 0;

    loop {
        let Some(&id) = ids.get(i) else {
            break;
        };

        let Some(&range) = id_ranges.get(j) else {
            break;
        };

        if id < range.min {
            // id is spoiled
            i += 1;
            continue;
        }

        if id <= range.max {
            sum += 1;
            i += 1;
        } else {
            j += 1;
        }
    }

    Ok(sum)
}

fn run_part_2(id_ranges: &[IdRange], ids: &[usize]) -> usize {
    42
}
