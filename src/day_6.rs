use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_6";

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

    ris_log::info!("run part 1...");
    let result = run_part_1(&input)?;
    answer.add(format!("1: {}", result));

    ris_log::info!("run part 2...");
    let result = run_part_2(&input)?;
    answer.add(format!("2: {}", result));

    Ok(())
}

fn run_part_1(input: &str) -> RisResult<usize> {
    ris_log::info!("parse input...");
    let mut lines = input.lines().collect::<Vec<_>>();
    let last = lines.len() - 1;
    let operations_str = lines.remove(last);

    ris_log::info!("parse numbers...");
    let mut number_lines = Vec::new();
    for line in lines {
        let mut number_line = Vec::new();

        let splits = line.split(' ');
        for split in splits {
            let split = split.trim();
            if split.is_empty() {
                continue;
            }

            let number = split.parse::<usize>()?;
            number_line.push(number);
        }

        number_lines.push(number_line);
    }

    ris_log::info!("parse operations...");
    let mut operations = Vec::new();
    let splits = operations_str.split(' ');
    for split in splits {
        let split = split.trim();
        if split.is_empty() {
            continue;
        }

        let operation = match split {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => return ris_error::new_result!("invalid operation: {}", split),
        };

        operations.push(operation);
    }

    ris_log::info!("build problems...");
    let mut problems = Vec::new();
    for &operation in operations.iter() {
        let problem = Problem {
            numbers: Vec::new(),
            operation,
        };
        problems.push(problem);
    }

    for number_line in number_lines.iter() {
        for (i, &number) in number_line.iter().enumerate() {
            problems[i].numbers.push(number);
        }
    }

    ris_log::info!("solve problems...");
    let mut sum = 0;
    for problem in problems {
        let result = problem.solve();
        sum += result
    }

    Ok(sum)
}

fn run_part_2(input: &str) -> RisResult<usize> {
    ris_log::info!("read character matrix...");
    let mut m = Vec::new();
    for line in input.lines() {
        let mut v = Vec::new();
        for c in line.chars() {
            v.push(c);
        }
        m.push(v);
    }

    ris_log::info!("transpose matrix...");
    let width = m[0].len();
    let height = m.len();

    let mut t = Vec::new();
    for ix in 0..width {
        let mut v = Vec::new();
        #[allow(clippy::needless_range_loop)]
        // justification: since we are indexing a matrix,
        // having both indices written out like this makes
        // the code much clearer
        for iy in 0..height {
            let c = m[iy][ix];
            v.push(c);
        }
        t.push(v);
    }

    ris_log::info!("parse matrix and build problem...");
    let mut problems = Vec::new();
    for v in t.iter() {
        let operation_char = v.last().into_ris_error()?;
        match *operation_char {
            '+' => {
                let problem = Problem {
                    numbers: Vec::new(),
                    operation: Operation::Addition,
                };
                problems.push(problem);
            }
            '*' => {
                let problem = Problem {
                    numbers: Vec::new(),
                    operation: Operation::Multiplication,
                };
                problems.push(problem);
            }
            ' ' => (),
            _ => return ris_error::new_result!("invalid operation: '{}'", operation_char),
        }

        let problem = problems.last_mut().into_ris_error()?;

        let mut number = 0usize;
        let digit_count = v.len() - 1;
        for &c in v.iter().take(digit_count) {
            if c == ' ' {
                continue;
            }

            let digit = c.to_digit(10).into_ris_error()?;
            number = number * 10 + digit as usize;
        }

        if number == 0 {
            continue;
        }

        problem.numbers.push(number);
    }

    ris_log::info!("solve problems...");
    let mut sum = 0;
    for problem in problems {
        let result = problem.solve();
        sum += result
    }

    Ok(sum)
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<usize>,
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> usize {
        match self.operation {
            Operation::Addition => {
                let mut sum = 0;
                for number in self.numbers.iter() {
                    sum += number;
                }

                sum
            }
            Operation::Multiplication => {
                let mut product = 1;
                for number in self.numbers.iter() {
                    product *= number;
                }

                product
            }
        }
    }
}
