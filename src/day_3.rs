use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_3";

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse input...");
    let mut banks = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let mut bank = Vec::new();
        for c in line.trim().chars() {
            let battery = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => return ris_error::new_result!("invalid battery: {}", c),
            };

            bank.push(battery);
        }

        banks.push(bank);
    }

    ris_log::info!("run part 1...");
    let result = run_part_1(&banks)?;
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&banks)?;
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(banks: &[Vec<usize>]) -> RisResult<usize> {
    let mut sum = 0;

    for bank in banks.iter() {
        let mut batteries = bank.iter().rev();
        let mut battery_2 = batteries.next().into_ris_error()?;
        let mut battery_1 = batteries.next().into_ris_error()?;
        for battery in batteries {
            if battery < battery_1 {
                continue;
            }

            if battery_1 >= battery_2 {
                battery_2 = battery_1;
            }

            battery_1 = battery;
        }

        let joltage = battery_1 * 10 + battery_2;
        sum += joltage;
    }

    Ok(sum)
}

fn run_part_2(banks: &[Vec<usize>]) -> RisResult<usize> {
    let mut sum = 0;

    for bank in banks.iter() {
        let mut batteries_to_check = bank.iter().rev();
        let mut batteries = Vec::with_capacity(12);
        for _ in 0..batteries.capacity() {
            let battery = batteries_to_check.next().into_ris_error()?;
            batteries.push(*battery);
        }

        let mut batteries = batteries.into_iter().rev().collect::<Vec<_>>();

        for &battery in batteries_to_check {
            if battery < batteries[0] {
                continue;
            }

            // produce ripple
            let mut new_batteries = Vec::with_capacity(batteries.capacity());
            new_batteries.push(battery);

            // ripple
            for i in new_batteries.len()..new_batteries.capacity() {
                let i1 = i - 1;
                let i2 = i;

                if batteries[i1] >= batteries[i2] {
                    new_batteries.push(batteries[i1]);
                } else {
                    break;
                }
            }

            #[allow(clippy::needless_range_loop)]
            // justification: the loop mirrors the one above, by continuing
            // where the upper left off. also, the suggested fix by clippy
            // is longer, and imo less understandable as it stands right now
            for i in new_batteries.len()..new_batteries.capacity() {
                new_batteries.push(batteries[i]);
            }

            batteries = new_batteries;
        }

        let mut joltage = 0;
        for (i, battery) in batteries.iter().rev().enumerate() {
            let mut power = 1;
            for _ in 0..i {
                power *= 10;
            }

            joltage += battery * power;
        }

        sum += joltage
    }

    Ok(sum)
}
