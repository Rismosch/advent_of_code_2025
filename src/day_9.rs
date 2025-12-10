use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_9";

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse_inputs...");
    let mut tiles = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut splits = line.split(',');
        let x_str = splits.next().into_ris_error()?;
        let y_str = splits.next().into_ris_error()?;
        let x = x_str.parse()?;
        let y = y_str.parse()?;
        let tile = Vec2(x, y);
        tiles.push(tile);
    }

    ris_log::info!("find largest area...");
    let mut max_area = usize::MIN;

    for (i, &a) in tiles.iter().enumerate() {
        for &b in tiles.iter().skip(i + 1) {
            let area = aabb_area(a, b);
            if area > max_area {
                max_area = area;
            }
        }
    }

    answer.add(format!("1: {}", max_area));

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Vec2(usize, usize);

fn aabb_area(a: Vec2, b: Vec2) -> usize {
    let ax = a.0 as isize;
    let ay = a.1 as isize;
    let bx = b.0 as isize;
    let by = b.1 as isize;
    let x = isize::abs(ax - bx) + 1;
    let y = isize::abs(ay - by) + 1;
    let area = x * y;
    area as usize
}
