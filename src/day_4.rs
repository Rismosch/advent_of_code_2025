use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_4";

const ROLL: char = '@';
const EMPTY: char = '.';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Roll,
    Empty,
}

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse input...");
    let mut shelf = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut entries = Vec::new();
        for c in line.chars() {
            let entry = match c {
                ROLL => Entry::Roll,
                EMPTY => Entry::Empty,
                _ => return ris_error::new_result!("unsupported character: '{}'", c),
            };

            entries.push(entry);
        }
        shelf.push(entries);
    }

    ris_log::info!("run part 1...");
    let result = run_part_1(&shelf);
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&mut shelf);
    answer.add(format!("2: {}", result));

    Ok(())

}

fn run_part_1(shelf: &[Vec<Entry>]) -> usize {
    let mut sum = 0;
    for (iy, entries) in shelf.iter().enumerate() {
        for (ix, &entry) in entries.iter().enumerate() {
            if entry == Entry::Empty {
                continue;
            }

            if can_access(shelf, ix, iy) {
                sum += 1;
            }
        }
    }

    sum
}

fn run_part_2(shelf: &mut [Vec<Entry>]) -> usize {
    let mut sum = 0;

    loop {
        let mut roll_was_removed = false;

        for iy in 0..shelf.len() {
            let end = shelf[iy].len();
            for ix in 0..end {
                let entry = shelf[iy][ix];
                if entry == Entry::Empty {
                    continue;
                }

                if !can_access(shelf, ix, iy) {
                    continue;
                }

                sum += 1;
                shelf[iy][ix] = Entry::Empty;
                roll_was_removed = true;
            }
        }

        if !roll_was_removed {
            break;
        }
    }

    sum
}

fn index_shelf(shelf: &[Vec<Entry>], ix: isize, iy: isize) -> Option<Entry> {
    if ix < 0 || iy < 0 {
        return None;
    }

    let entries = shelf.get(iy as usize)?;
    let entry = entries.get(ix as usize)?;
    Some(*entry)
}

fn can_access(shelf: &[Vec<Entry>], ix: usize, iy: usize) -> bool {
    let offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut neighbors = 0;
    for offset in offsets.iter() {
        let ix = ix as isize + offset.0;
        let iy = iy as isize + offset.1;
        let entry = index_shelf(&shelf, ix, iy);
        if entry == Some(Entry::Roll) {
            neighbors += 1;
        }
    }

    neighbors < 4
}

