use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_10";

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse_inputs...");
    let mut machines = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut splits = line.split(' ');
        let lights_str = splits.next().into_ris_error()?;
        let mut splits = splits.rev();
        let joltages_str = splits.next().into_ris_error()?;
        let splits = splits.rev();
        let button_strs = splits;

        // parse lights
        let mut lights = Vec::with_capacity(lights_str.len() - 2);
        for c in lights_str.chars().skip(1).take(lights.capacity()) {
            let light = match c {
                '.' => false,
                '#' => true,
                _ => return ris_error::new_result!("invalid light: {}", c),
            };

            lights.push(light);
        }

        // parse joltage
        let start = 1;
        let end = joltages_str.len() - 1;
        let joltages_str = &joltages_str[start..end];
        let joltage_strs = joltages_str.split(',');
        let mut joltages = Vec::new();
        for joltage_str in joltage_strs {
            let joltage = joltage_str.parse()?;
            joltages.push(joltage);
        }

        // parse buttons
        let mut buttons = Vec::new();
        for button_str in button_strs {
            let start = 1;
            let end = button_str.len() - 1;
            let button_str = &button_str[start..end];
            let splits = button_str.split(',');
            let mut button = Vec::new();
            for split in splits {
                let parsed = split.parse()?;
                button.push(parsed);
            }
            buttons.push(button);
        }

        // construct machine
        let machine = Machine{
            lights,
            buttons,
            joltages,
        };

        machines.push(machine);
    }

    ris_log::info!("run part 1...");
    let result = run_part_1(&machines)?;
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&machines)?;
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(machines: &[Machine]) -> RisResult<usize> {
    let mut sum = 0;

    for machine in machines.iter() {
        let mut visited_nodes = std::collections::HashSet::<Lights>::new();
        let mut to_visit = std::collections::VecDeque::new();
        to_visit.push_back((machine.lights.clone(), 0));

        let mut shortest_path = None;

        while let Some((node, generation)) = to_visit.pop_front() {
            let was_inserted = visited_nodes.insert(node.clone());
            if !was_inserted {
                continue;
            }

            let new_generation = generation + 1;

            for button in machine.buttons.iter() {
                let new_node = press_button_1(button, &node);

                let is_turned_off = new_node.iter().all(|x| !x);
                if is_turned_off {
                    shortest_path = Some(new_generation);
                    break;
                }

                to_visit.push_back((new_node, new_generation));
            }

            if shortest_path.is_some() {
                break;
            }
        }

        let shortest_path = shortest_path.into_ris_error()?;
        sum += shortest_path
    }

    Ok(sum)
}

fn run_part_2(machines: &[Machine]) -> RisResult<usize> {
    use std::sync::atomic::AtomicUsize;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;

    std::thread::scope(|s| {
        let sum = Arc::new(AtomicUsize::new(0));
        let num_threads = 12;
        let progress = Arc::new(AtomicUsize::new(0));

        for i in 0..num_threads {
            let sum = sum.clone();
            let progress = progress.clone();
            s.spawn(move || {
                for machine in machines.iter().skip(i).step_by(num_threads) {
                    // log progress
                    let progress = progress.fetch_add(1, Ordering::Relaxed);
                    let percentage = 100.0 * progress as f32 / machines.len() as f32;
                    ris_log::info!(
                        "run machine... {}/{} {}%",
                        progress,
                        machines.len(),
                        percentage,
                    );

                    // run machine
                    let steps = configure_machine(machine);
                    sum.fetch_add(steps, Ordering::Relaxed);
                }
            });
        }

        let result = sum.load(Ordering::Relaxed);
        Ok(result)
    })
}


type Lights = Vec<bool>;
type Button = Vec<usize>;
type Joltages = Vec<usize>;

#[derive(Debug)]
struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    joltages: Joltages,
}

fn press_button_1(button: &Button, lights: &Lights) -> Lights {
    let mut lights = lights.clone();
    for &index in button.iter() {
        let light = &mut lights[index];
        *light = !*light;
    }
    lights
}

fn press_button_2(button: &Button, joltages: &Joltages) -> Option<Joltages> {
    let mut joltages = joltages.clone();
    for &index in button.iter() {
        let joltage = &mut joltages[index];
        *joltage = joltage.checked_sub(1)?;
    }

    Some(joltages)
}

fn configure_machine(machine: &Machine) -> usize {
    let mut visited_nodes = std::collections::HashSet::<Joltages>::new();
    let mut to_visit = std::collections::VecDeque::new();

    let mut buttons = machine.buttons.clone();
    let mut joltage = machine.joltages.clone();

    // decrease joltage by buttons, that are the only ones of changing a specific index. since
    // these buttons are the only one that can affect a specific index, they must be pressed
    // exactly as often as the joltage at the index. doing this upfront means we don't need to
    // search later, thus increasing performance.
    let mut generation = 0;

    loop {
        break;
        let mut button_index_map = Vec::with_capacity(joltage.len());
        for _ in 0..button_index_map.capacity() {
            button_index_map.push(Vec::new());
        }

        for button in buttons.clone().into_iter().enumerate() {
            for index in button.1.iter() {
                button_index_map[*index].push(button.clone())
            }
        }

        let mut buttons_to_remove = std::collections::HashSet::new();
        for entry in button_index_map.iter() {
            if entry.len() != 1 {
                continue;
            }

            let (i, button) = &entry[0];
            joltage = press_button_2(&button, &joltage).expect("operation to be valid");
            buttons_to_remove.insert(i);
            generation += 1;
        }

        if buttons_to_remove.is_empty() {
            // no buttons exist anymore that affect only one index
            break;
        }

        let mut buttons_to_remove = buttons_to_remove.iter().collect::<Vec<_>>();
        buttons_to_remove.sort();
        println!("remove {:?} buttons: {:?}", buttons_to_remove, buttons);
        for &&index in buttons_to_remove.iter().rev() {
            buttons.swap_remove(*index);
        }

    }

    to_visit.push_back((joltage, generation));

    let mut shortest_path = None;

    while let Some((node, generation)) = to_visit.pop_front() {
        let was_inserted = visited_nodes.insert(node.clone());
        if !was_inserted {
            continue;
        }

        let new_generation = generation + 1;

        for button in machine.buttons.iter() {
            let Some(new_node) = press_button_2(button, &node) else {
                continue; // invalid joltage reached
            };

            let is_turned_off = new_node.iter().all(|x| *x == 0);
            if is_turned_off {
                shortest_path = Some(new_generation);
                break;
            }

            to_visit.push_back((new_node, new_generation));
        }

        if shortest_path.is_some() {
            break;
        }
    }

    shortest_path.expect("a path to be found")
}

