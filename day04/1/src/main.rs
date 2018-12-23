extern crate regex;

use std::collections::HashMap;
use std::io::{self, BufRead};

use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let entries : Vec<LogEntry> = handle.lines().map(|x| x.unwrap()).sorted().map(|x| parse_line(&x)).collect();

    let mut guard_id = 0;
    let mut sleep_hour = 99;
    let mut sleep_min = 99;
    let mut mins_asleep_count : HashMap<i32, u32> = HashMap::new();
    let mut mins_asleep : HashMap<(i32, u8), u32> = HashMap::new();  // (guard_id, minute (0-59)) -> count
    for entry in entries {
        match entry.transition {
            GuardState::BeginsShift(id) => {
                guard_id = id;
            }
            GuardState::FallsAsleep => {
                sleep_hour = entry.hour;
                sleep_min = entry.min;
            }
            GuardState::WakesUp => {
                assert!(sleep_hour == entry.hour);
                let mins = entry.min - sleep_min;

                if let Some(result) = mins_asleep_count.get_mut(&guard_id) {
                    *result = *result + mins as u32;
                } else {
                    mins_asleep_count.insert(guard_id, mins as u32);
                }


                for i in sleep_min..entry.min {
                    if let Some(count) = mins_asleep.get_mut(&(guard_id, i)) {
                        *count += 1;
                    } else {
                        mins_asleep.insert((guard_id, i), 1);
                    }
                }
            }
        }
    }

    let (guard_id, sleep_time) = mins_asleep_count.iter().max_by_key(|i| i.1).unwrap();
    println!("Guard #{} slept the most @ {} minutes.", guard_id, sleep_time);

    let ((_, min), count) = mins_asleep.iter().filter(|i| (i.0).0 == *guard_id).max_by_key(|i| i.1).unwrap();

    println!("Guard #{} was asleep at 00:{:2} {} times", guard_id, min, count);

}

// Example inputs:
// [1518-11-01 00:00] Guard #10 begins shift
// [1518-11-01 00:05] falls asleep
// [1518-11-01 00:25] wakes up
fn parse_line(line: &str) -> LogEntry {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d\d):(\d\d)\] (.+)").unwrap();
    }
    let captures = RE.captures(line).unwrap();

    let year = captures[1].parse().unwrap();
    let month = captures[2].parse().unwrap();
    let day = captures[3].parse().unwrap();
    let hour = captures[4].parse().unwrap();
    let min = captures[5].parse().unwrap();

    let msg = &captures[6];

    return LogEntry{year, month, day, hour, min, transition: get_guard_state(msg)};
}

struct LogEntry {
    year: u32,
    month: u8,
    day: u8,

    hour: u8,
    min: u8,

   transition: GuardState,
}

fn get_guard_state(msg: &str) -> GuardState {
    if msg.contains("Guard") {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        }
        let id = RE.captures(msg).unwrap()[1].parse().unwrap();
        // Guard #99 begins shift
        GuardState::BeginsShift(id)
    } else if msg.contains("falls asleep") {
        GuardState::FallsAsleep
    } else {
        GuardState::WakesUp
    }
}

enum GuardState {
    BeginsShift(i32),
    FallsAsleep,
    WakesUp,
}
