use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_1";
const POSITIONS: usize = 100;

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("parse input...");
    let mut rotations = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let rotation = Rotation::try_from(line)?;
        rotations.push(rotation);
    }

    ris_log::info!("run part 1...");
    let result = run_part_1(&rotations)?;
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&rotations)?;
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(input: &[Rotation]) -> RisResult<usize> {
    ris_log::info!("apply rotations...");
    let mut dial = Dial::<POSITIONS> { position: 50 };

    let mut counter = 0;

    for &rotation in input.iter() {
        dial.add(rotation)?;

        if dial.position == 0 {
            counter += 1;
        }
    }

    Ok(counter)
}

fn run_part_2(input: &[Rotation]) -> RisResult<usize> {
    ris_log::info!("apply rotations...");
    let mut dial = Dial::<POSITIONS> { position: 50 };

    let mut counter = 0;
    for &rotation in input.iter() {
        let Rotation {
            direction,
            clicks,
        } = rotation;

        // surely there's a better way than this, but
        // the input isn't that big, and it takes about
        // a millisecond on my machine, so what gives?
        for _ in 0..clicks {
            dial.add(Rotation {
                direction,
                clicks: 1,
            })?;

            if dial.position == 0 {
                counter += 1;
            }
        }
    }

    Ok(counter)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    clicks: usize,
}

#[derive(Debug, Clone, Copy)]
struct Dial<const CLICKS: usize> {
    position: usize,
}

impl<const T: usize> Dial<T> {
    fn add(&mut self, rotation: Rotation) -> RisResult<()> {
        let clicks_to_apply = rotation.clicks % T;

        match rotation.direction {
            Direction::Left => {
                if clicks_to_apply > self.position {
                    self.position += T - clicks_to_apply;
                } else {
                    self.position -= clicks_to_apply;
                }
            }
            Direction::Right => {
                self.position = self
                    .position
                    .checked_add(clicks_to_apply)
                    .into_ris_error()?;

                if self.position >= T {
                    self.position -= T;
                }
            }
        }

        Ok(())
    }
}

impl TryFrom<&str> for Rotation {
    type Error = RisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction_str = &value[0..1];
        let clicks_str = &value[1..];

        let direction = match direction_str.to_lowercase().as_str() {
            "l" => Direction::Left,
            "r" => Direction::Right,
            _ => return ris_error::new_result!("invalid direction: {}", direction_str),
        };

        let clicks = clicks_str.parse()?;

        Ok(Rotation { direction, clicks })
    }
}
