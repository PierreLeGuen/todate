use chrono::{Local, TimeZone};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <timestamp>", args[0]);
        return;
    }

    let timestamp: i64 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: Invalid timestamp");
            return;
        }
    };

    let datetime_local = match args[1].len() {
        10 => Local.timestamp_opt(timestamp, 0),
        13 => {
            let seconds = timestamp / 1000;
            let subsec_millis = (timestamp % 1000) as u32;
            Local.timestamp_opt(seconds, subsec_millis * 1_000_000)
        }
        19 => {
            let seconds = timestamp / 1_000_000_000;
            let subsec_nanos = (timestamp % 1_000_000_000) as u32;
            Local.timestamp_opt(seconds, subsec_nanos)
        }
        _ => {
            eprintln!("Error: Unrecognized timestamp precision. Please provide a timestamp in seconds, milliseconds, or nanoseconds.");
            return;
        }
    };

    println!("{}", datetime_local.unwrap().to_rfc3339());
}
