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

            let line = VerticalLine {
                x: t1.0,
                ya: t1.1,
                yb: t2.1,
            };
            vertical_lines.push(line);
        } else if t1.1 == t2.1 {
            // horizontal
            if t2.0 < t1.0 {
                std::mem::swap(&mut t1, &mut t2);
            }

            let line = HorizontalLine {
                xa: t1.0,
                xb: t2.0,
                y: t1.1,
            };
            horizontal_lines.push(line);
        } else {
            return ris_error::new_result!(
                "t1 {:?} and t2 {:?} must align vertically or horizontally",
                t1,
                t2,
            );
        }
    }

    // find largest rectangle
    let mut max_area = usize::MIN;

    for (i, &a) in tiles.iter().enumerate() {
        for &b in tiles.iter().skip(i + 1) {
            let aabb = Aabb::new(a, b);

            let mut intersects_with_line = false;

            // check vertical
            for &VerticalLine { x, ya, yb } in vertical_lines.iter() {
                intersects_with_line = 
                    x > aabb.min.0 &&
                    x < aabb.max.0 &&
                    ya < aabb.max.1 &&
                    yb > aabb.min.1;

                if intersects_with_line {
                    break;
                }
            }

            if intersects_with_line {
                continue;
            }

            // check horizontal
            for &HorizontalLine { xa, xb, y } in horizontal_lines.iter() {
                intersects_with_line = 
                    xa < aabb.max.0 &&
                    xb > aabb.min.0 &&
                    y > aabb.min.1 &&
                    y < aabb.max.1;

                if intersects_with_line {
                    break;
                }
            }

            if intersects_with_line {
                continue;
            }

            // compute area
            let area = aabb.area();
            if area > max_area {
                max_area = area;
            }
        }
    }

    Ok(max_area)
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
struct VerticalLine {
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
        tile.0 == self.x && tile.1 >= self.ya && tile.1 <= self.yb
    }

    fn ends(self) -> [Vec2; 2] {
        [Vec2(self.x, self.ya), Vec2(self.x, self.yb)]
    }
}

impl Line for HorizontalLine {
    fn collides_with(self, tile: Vec2) -> bool {
        tile.0 >= self.xa && tile.0 <= self.xb && tile.1 == self.y
    }

    fn ends(self) -> [Vec2; 2] {
        [Vec2(self.xa, self.y), Vec2(self.xb, self.y)]
    }
}
