use ris_error::prelude::*;

const PUZZLE_INPUT_KEY: &str = "day_6";

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
            },
            Operation::Multiplication => {
                let mut product = 1;
                for number in self.numbers.iter() {
                    product *= number;
                }

                product
            },
        }
    }
}

pub fn run(answer: &mut crate::Answer) -> RisResult<()> {
    ris_log::info!("read input...");
    let input = crate::read_puzzle_input(PUZZLE_INPUT_KEY)?;

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
        let problem = Problem{
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

    ris_log::info!("run part 1...");
    let result = run_part_1(&problems);
    answer.add(format!("1: {}", result));

    Ok(())
}

fn run_part_1(problems: &[Problem]) -> usize {
    let mut sum = 0;
    for problem in problems {
        let result = problem.solve();
        sum += result
    }
    sum
}
