use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_9_custom";

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

    ris_log::info!("run part 1...");
    let result = run_part_1(&tiles);
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&tiles)?;
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(tiles: &[Vec2]) -> usize {
    let mut max_area = usize::MIN;

    for (i, &a) in tiles.iter().enumerate() {
        for &b in tiles.iter().skip(i + 1) {
            let aabb = Aabb::new(a, b);
            let area = aabb.area();
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn run_part_2(tiles: &[Vec2]) -> RisResult<usize> {
    // find biggest aabb
    ris_log::info!("find global aabb...");
    let mut tiles_iter = tiles.iter();
    let t1 = *tiles_iter.next().into_ris_error()?;
    let t2 = *tiles_iter.next().into_ris_error()?;
    let mut global_aabb = Aabb::new(t1, t2);

    for &Vec2(x, y) in tiles_iter {
        global_aabb.min.0 = usize::min(global_aabb.min.0, x);
        global_aabb.max.0 = usize::max(global_aabb.max.0, x);
        global_aabb.min.1 = usize::min(global_aabb.min.1, y);
        global_aabb.max.1 = usize::max(global_aabb.max.1, y);
    }

    ris_log::info!("global aabb: {:?}", global_aabb);

    // parse lines
    ris_log::info!("parse lines...");
    let mut vertical_lines = Vec::new();
    let mut horizontal_lines = Vec::new();

    for (i, t1) in tiles.iter().enumerate() {
        let j = (i + 1) % tiles.len();

        let mut t1 = *t1;
        let mut t2 = tiles[j];

        // construct line
        if t1 == t2 {
            return ris_error::new_result!("t1 and t2 may not be equal");
        }

        if t1.0 == t2.0 {
            // vertial
            if t2.1 < t1.1 {
                std::mem::swap(&mut t1, &mut t2);
            }

            let line = VerticalLine{ x: t1.0, ya: t1.1, yb: t2.1 };
            vertical_lines.push(line);
        } else if t1.1 == t2.1 {
            // horizontal
            if t2.0 < t1.0 {
                std::mem::swap(&mut t1, &mut t2);
            }

            let line = HorizontalLine{ xa: t1.0, xb: t2.0, y: t1.1 };
            horizontal_lines.push(line);
        } else {
            return ris_error::new_result!(
                "t1 {:?} and t2 {:?} must align vertically or horizontally",
                t1,
                t2,
            );
        }
    }

    // find holes
    ris_log::info!("find holes...");

    let Aabb {
        min: Vec2(x_start, y_start),
        max: Vec2(x_end, y_end),
    } = global_aabb;
    let x_end = x_end + 1; // bounds are inclusive
    let y_end = y_end + 1;

    let mut current_aabb_index = None::<usize>;

    //let mut holes = Vec::new();
    //let mut holes_x = Vec::<Aabb>::new();
    //let mut holes_x_previous = Vec::<Aabb>::new();

    for y in y_start..y_end {
        
        //let mut holes_x = Vec::new();

        // find all collisions
        let mut vertical_collisions = Vec::new();
        let mut horizontal_collisions = Vec::new();

        for x in x_start..x_end {
            let tile = Vec2(x, y);

            for line in vertical_lines.iter() {
                if line.collides_with(tile) {
                    vertical_collisions.push(line);
                }
            }

            for line in horizontal_lines.iter() {
                if line.collides_with(tile) {
                    horizontal_collisions.push(line);
                }
            }
        }
        
        // sort collisions, for faster iteration later
        vertical_collisions.sort_by(|lhs, rhs| lhs.x.cmp(&rhs.x));
        horizontal_collisions.sort_by(|lhs, rhs| lhs.xa.cmp(&rhs.xb));

        // determine whether the ends of the horizontal line point in the same direction or not
        //
        // example of same direction:
        //
        //     |   |
        //     +---+
        //
        // example of different direction:
        //
        //     |    
        //     +---+
        //         |
        let mut horizontal_collisions = horizontal_collisions.iter()
            .map(|horizontal_line| {
                #[derive(Debug, PartialEq, Eq)]
                enum Direction {
                    Up,
                    Down,
                }

                let mut end_directions = Vec::with_capacity(2);
                for vertical_line in vertical_collisions.iter() {
                    let [v0, v1] = vertical_line.ends();
                    let [h0, h1] = horizontal_line.ends();

                    // on construction we made sure that the a coordinate is smaller than the b
                    // coordinate. thus, depending what tile matches, we can determine the
                    // direction of the end
                    if h0 == v0 {
                        end_directions.push(Direction::Down);
                    } else if h0 == v1 {
                        end_directions.push(Direction::Up);
                    } else if h1 == v0 {
                        end_directions.push(Direction::Down);
                    } else if h1 == v1 {
                        end_directions.push(Direction::Up);
                    }

                    if end_directions.len() == 2 {
                        break;
                    }
                }

                //println!("y {} {:?} {:#?}", y, end_directions, vertical_collisions);
                let ends_have_same_direction = end_directions[0] == end_directions[1];
                (horizontal_line, ends_have_same_direction)
            })
            .collect::<Vec<_>>();

            // find holes
            let mut holes_x = Vec::new();

            let mut horizontal_line_index = if horizontal_collisions.is_empty() {
                None
            } else {
                Some(0)
            };
            let mut is_inside = false;
            for j in 1..vertical_collisions.len() {
                let i = j - 1;
                let xa = vertical_collisions[i].x;
                let xb = vertical_collisions[j].x;
                let hole = HorizontalLine{ xa, xb, y };

                match horizontal_line_index.as_mut() {
                    Some(ih) => {
                        let (horizontal_line, ends_have_same_direction) = horizontal_collisions[*ih];
                        if hole == **horizontal_line {
                            *ih += 1;
                            if *ih >= horizontal_collisions.len() {
                                horizontal_line_index = None;
                            }

                            if !ends_have_same_direction {
                                is_inside = !is_inside;
                            }
                        }
                    },
                    None => {
                        is_inside = !is_inside;

                        if !is_inside {
                            holes_x.push(hole);
                        }
                    },
                }
            }

        ris_log::trace!("{} holes: {:#?}", y, holes_x);

        current_aabb_index = None;
    }
    
    // find largest rectangle

    Ok(42)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2(usize, usize);

#[derive(Debug, Clone, Copy)]
struct Aabb {
    min: Vec2,
    max: Vec2,
}

impl Aabb {
    fn new(t1: Vec2, t2: Vec2) -> Self {
        let (min_x, max_x) = if t1.0 < t2.0 {
            (t1.0, t2.0)
        } else {
            (t2.0, t1.0)
        };

        let (min_y, max_y) = if t1.1 < t2.1 {
            (t1.1, t2.1)
        } else {
            (t2.1, t1.1)
        };

        Self {
            min: Vec2(min_x, min_y),
            max: Vec2(max_x, max_y),
        }
    }

    fn area(self) -> usize {
        let x = self.max.0 - self.min.0 + 1;
        let y = self.max.1 - self.min.1 + 1;
        x * y
    }
}

trait Line {
    fn collides_with(self, tile: Vec2) -> bool;
    fn ends(self) -> [Vec2; 2];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VerticalLine
{
    x: usize,
    ya: usize,
    yb: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HorizontalLine {
    xa: usize,
    xb: usize,
    y: usize,
}

impl Line for VerticalLine {
    fn collides_with(self, tile: Vec2) -> bool {
        tile.0 == self.x &&
            tile.1 >= self.ya &&
            tile.1 <= self.yb
    }

    fn ends(self) -> [Vec2; 2] {
        [
            Vec2(self.x, self.ya),
            Vec2(self.x, self.yb),
        ]
    }
}

impl Line for HorizontalLine {
    fn collides_with(self, tile: Vec2) -> bool {
        tile.0 >= self.xa &&
            tile.0 <= self.xb &&
            tile.1 == self.y
    }

    fn ends(self) -> [Vec2; 2] {
        [
            Vec2(self.xa, self.y),
            Vec2(self.xb, self.y),
        ]
    }
}

