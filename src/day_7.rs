use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_7";

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse input...");
    let mut lines = input.lines();

    ris_log::info!("parse start...");
    let start_str = lines.next().into_ris_error()?;
    let (start, _) = start_str.chars()
        .enumerate()
        .find(|(_, c) | *c == 'S')
        .into_ris_error()?;

    ris_log::info!("parse manifold...");
    let mut manifold = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut splitters = Vec::new();
        let mut constains_splitter = false;
        for c in line.trim().chars() {
            let is_splitter = c == '^';
            constains_splitter |= is_splitter;
            splitters.push(is_splitter);
        }

        if constains_splitter {
            manifold.push(splitters);
        }
    }

    ris_log::info!("run part 1...");
    let result = run_part_1(&manifold, start);
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&manifold, start);
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(manifold: &[Vec<bool>], start: usize) -> usize {
    let mut sum = 0;

    let width = manifold[0].len();
    let mut state = vec![false; width];
    state[start] = true;

    for splitters in manifold.iter() {
        let mut new_state = vec![false; width];

        for (i, &beam) in state.iter().enumerate() {
            if !beam {
                continue;
            }

            let is_splitter = splitters[i];
            if !is_splitter {
                new_state[i] = true;
            } else {
                new_state[i - 1] = true;
                new_state[i + 1] = true;
                sum += 1;
            }
        }

        state = new_state;
    }

    sum
}

fn run_part_2(manifold: &[Vec<bool>], start: usize) -> usize {
    let width = manifold[0].len();
    let mut state = vec![1; width];

    for splitters in manifold.iter().rev() {
        let mut new_state = vec![0; width];
        for (i, &beam) in state.iter().enumerate() {
            if beam == 0 {
                continue;
            }

            // handle split
            let is_on_left_edge = i == 0;
            let is_on_right_edge = i == width - 1;

            let is_splitter_left = !is_on_left_edge && splitters[i - 1];
            let is_splitter_right = !is_on_right_edge && splitters[i + 1];

            if is_splitter_left {
                new_state[i - 1] += beam
            }

            if is_splitter_right {
                new_state[i + 1] += beam
            }

            // passthrough
            let is_splitter = splitters[i];
            if !is_splitter {
                new_state[i] += beam;
            }
        }

        state = new_state;
    }

    state[start]
}
