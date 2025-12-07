mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

use std::io::Read;
use std::path::PathBuf;

use ris_error::prelude::*;
use ris_log::constructed_log_message::ConstructedLogFormatArgs;
use ris_log::log::IAppender;
use ris_log::log::LogGuard;
use ris_log::log_level::LogLevel;
use ris_log::log_message::LogMessage;

const PUZZLE_INPUT_PATH: &str = "puzzle_input";

const LOG_LEVEL: LogLevel = LogLevel::Trace;

struct ConsoleAppender;

impl IAppender for ConsoleAppender {
    fn print(&mut self, message: &LogMessage) {
        let args = ConstructedLogFormatArgs {
            ansi_support: true,
            show_timestamp: false,
            show_priority: true,
            show_foot: false,
        };
        let message_string = message.fmt(args);
        eprintln!("{}", message_string);
    }
}

pub fn read_puzzle_input(key: impl AsRef<str>) -> RisResult<String> {
    let path = PathBuf::from(PUZZLE_INPUT_PATH).join(key.as_ref());
    let mut file = std::fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Default)]
struct Answer(Vec<String>);

impl Answer {
    pub fn add(&mut self, message: impl AsRef<str>) {
        self.0.push(message.as_ref().to_string());
    }
}

fn main() -> RisResult<()> {
    let start = std::time::Instant::now();

    // init logging
    let console_appender = Box::new(ConsoleAppender);
    let appenders: Vec<Box<dyn IAppender + Send>> = vec![console_appender];
    let log_guard = ris_log::log::init(LOG_LEVEL, appenders);

    // parse args
    let raw_args = std::env::args().collect::<Vec<_>>();
    if raw_args.len() < 2 {
        return print_usage(log_guard, "too few arguments");
    }

    if raw_args.len() > 2 {
        return print_usage(log_guard, "too many arguments");
    }

    let day = raw_args[1].trim();

    // run
    let mut answer = Answer::default();

    let day_callbacks = [
        day_1::run,
        day_2::run,
        day_3::run,
        day_4::run,
        day_5::run,
        day_6::run,
        day_7::run,
    ];

    let mut run_day = |number: usize| {
        let callback = day_callbacks.get(number - 1).into_ris_error()?;
        callback(&mut answer)
    };

    match day {
        "1" => run_day(1)?,
        "2" => run_day(2)?,
        "3" => run_day(3)?,
        "4" => run_day(4)?,
        "5" => run_day(5)?,
        "6" => run_day(6)?,
        "7" => run_day(7)?,
        "all" => {
            for (i, callback) in day_callbacks.iter().enumerate() {
                let day_number = i + 1;
                ris_log::info!("run day {}...", day_number);
                answer.add(format!("day {}:", day_number));
                if let Err(e) = callback(&mut answer) {
                    ris_log::error!("day {} failed: {:?}", day_number, e);
                    answer.add(format!("error: {}", e.message));
                };
                answer.add(String::new());
            }
        }
        _ => return print_usage(log_guard, format!("invalid day number: {}", day)),
    }

    // print output
    drop(log_guard);
    eprintln!();
    eprintln!("answers:");
    for message in answer.0 {
        println!("{}", message);
    }

    // print time
    let end = std::time::Instant::now();
    let duration = end - start;
    eprintln!();
    eprintln!("done! time elapsed: {:?}", duration);

    Ok(())
}

fn print_usage(log_guard: LogGuard, message: impl AsRef<str>) -> RisResult<()> {
    ris_log::error!("{}", message.as_ref());
    drop(log_guard);

    eprintln!();
    eprintln!("usage:");
    eprintln!("\tcargo run -r <day number>");
    eprintln!();
    eprintln!("pass `all` to run all days");

    Ok(())
}
