mod day_1;

use std::io::Read;
use std::path::PathBuf;

use ris_error::prelude::*;
use ris_log::constructed_log_message::ConstructedLogFormatArgs;
use ris_log::log::IAppender;
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
    // init logging
    let console_appender = Box::new(ConsoleAppender);
    let appenders: Vec<Box<dyn IAppender + Send>> = vec![console_appender];
    let log_guard = ris_log::log::init(LOG_LEVEL, appenders);

    // parse args
    let raw_args = std::env::args().collect::<Vec<_>>();
    if raw_args.len() < 2 {
        return print_usage("too few arguments");
    }

    if raw_args.len() > 2 {
        return print_usage("too many arguments");
    }

    let day = raw_args[1].trim();

    // run
    let mut answer = Answer::default();

    match day {
        "1" => day_1::run(&mut answer)?,
        _ => return print_usage(format!("invalid day number: {}", day)),
    }

    // print output
    drop(log_guard);
    eprintln!();
    eprintln!("answer:");
    for message in answer.0 {
        println!("{}", message);
    }

    Ok(())
}

fn print_usage(message: impl AsRef<str>) -> RisResult<()> {
    ris_log::error!("{}", message.as_ref());

    let mut message = String::new();
    message.push_str("\nusage:");
    message.push_str("\n\tcargo run -r -- <day number>");

    let log_message = LogMessage::Plain(message);
    ris_log::log::forward_to_appenders(log_message);

    Ok(())
}
