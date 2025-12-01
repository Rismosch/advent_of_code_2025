use ris_error::prelude::*;

const POSITIONS: usize = 100;

const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

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
                    self.position = T - clicks_to_apply + self.position;
                } else {
                    self.position -= clicks_to_apply;
                }
            },
            Direction::Right => {
                self.position = self.position
                    .checked_add(clicks_to_apply)
                    .into_ris_error()?;

                if self.position >= T {
                    self.position -= T;
                }
            },
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

        Ok(Rotation {direction, clicks})
    }
}

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input("day_1")?;
    //let input = TEST_INPUT;

    let mut rotations = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let rotation = Rotation::try_from(line)?;
        rotations.push(rotation);
    }

    ris_log::info!("apply rotations...");
    let mut dial = Dial::<POSITIONS> {
        position: 50,
    };

    let mut counter = 0;

    ris_log::trace!("position: {}", dial.position);
    for rotation in rotations {
        dial.add(rotation)?;
        ris_log::trace!("position: {}", dial.position);

        if dial.position == 0 {
            counter += 1;
        }
    }

    answer.add(counter.to_string());

    Ok(())
}
