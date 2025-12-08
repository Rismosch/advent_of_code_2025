use std::collections::HashSet;

use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_8";
const CONNECTIONS: usize = 1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec3(isize, isize, isize);

#[derive(Debug, Clone, Copy)]
struct Connection {
    a: Vec3,
    b: Vec3,
    squared_magnitude: usize,
}

impl Connection {
    fn new(a: Vec3, b: Vec3) -> Self {
        let d = Vec3(
            a.0 - b.0,
            a.1 - b.1,
            a.2 - b.2,
        );

        let squared_magnitude = (d.0 * d.0 + d.1 * d.1 + d.2 * d.2) as usize;

        Self {
            a,
            b,
            squared_magnitude,
        }
    }
}

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse input...");
    let mut boxes = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut splits = line.split(',');
        let x_str = splits.next().into_ris_error()?;
        let y_str = splits.next().into_ris_error()?;
        let z_str = splits.next().into_ris_error()?;

        let x = x_str.parse()?;
        let y = y_str.parse()?;
        let z = z_str.parse()?;
    
        let p = Vec3(x, y, z);
        boxes.push(p);
    }
    
    ris_log::info!("compute connections...");
    let mut all_possible_connections = Vec::new();
    for (i, &a) in boxes.iter().enumerate() {
        for &b in boxes.iter().skip(i + 1) {
            let connection = Connection::new(a, b);
            all_possible_connections.push(connection);
        }
    }

    ris_log::info!("sort connections...");
    all_possible_connections.sort_by(|lhs, rhs| lhs.squared_magnitude.cmp(&rhs.squared_magnitude));

    ris_log::info!("build circuits...");
    let mut connection_iter = all_possible_connections.iter();
    let mut circuits: Vec<HashSet<Vec3>> = Vec::new();
    for _ in 0..CONNECTIONS {
        let connection = *connection_iter.next().into_ris_error()?;
        connect(&mut circuits, connection);
    }

    ris_log::info!("sort circuits...");
    circuits.sort_by(|lhs, rhs| rhs.len().cmp(&lhs.len()));

    ris_log::info!("compute answer 1...");
    let product = circuits.iter()
        .map(|x| x.len())
        .take(3)
        .fold(1, |acc, x| acc * x);
    answer.add(format!("1: {}", product));

    ris_log::info!("continue building circuits...");
    let mut product = None;
    while let Some(&connection) = connection_iter.next() {
        connect(&mut circuits, connection);

        // consider breaking early
        let is_one_circuit = circuits.len() == 1;
        if !is_one_circuit {
            continue;
        }

        let connected_everything = circuits[0].len() == boxes.len();
        if !connected_everything {
            continue;
        }

        ris_log::info!("last connection found!");
        let Connection { a, b, squared_magnitude: _ } = connection;
        product = Some(a.0 * b.0);
        break;
    }

    let product_string = match product {
        Some(product) => product.to_string(),
        None => "None".to_string(),
    };
    answer.add(format!("2: {}", product_string));

    Ok(())
}

fn connect(
    circuits: &mut Vec<HashSet<Vec3>>,
    connection: Connection,
) {
    let Connection { a, b, squared_magnitude: _ } = connection;

    // a connection may bridge two existing circuits, so we search for two circuits
    let mut candidates = Vec::with_capacity(2);

    let mut skip = false;

    for (i, circuit) in circuits.iter_mut().enumerate() {
        let contains_a = circuit.contains(&a);
        let contains_b = circuit.contains(&b);

        skip = contains_a && contains_b;
        if skip {
            break;
        }

        let can_insert = contains_a || contains_b;
        if can_insert {
            candidates.push(i);
        }

        if candidates.len() == 2 {
            break;
        }
    }

    if skip {
        // nothing to do, since one circuit already contains both boxes
        return;
    }

    match candidates.len() {
        // no candidates, make new circuit
        0 => {
            let mut circuit = HashSet::new();
            circuit.insert(a);
            circuit.insert(b);
            circuits.push(circuit);
        },
        // exactly one candidate, insert
        1 => {
            let candidate = candidates[0];
            let circuit = &mut circuits[candidate];
            circuit.insert(a);
            circuit.insert(b);
        },
        // connection will bridge the candidates, merge circuits
        _ => {
            let circuit_1 = circuits.swap_remove(candidates[1]);
            let circuit_0 = &mut circuits[candidates[0]];
            for junction_box in circuit_1 {
                circuit_0.insert(junction_box);
            }
        },
    }
}
