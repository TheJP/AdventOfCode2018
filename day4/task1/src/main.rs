extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use regex::Regex;

enum Event {
    Wake,
    Sleep,
    Begin,
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    // [1518-08-23 23:56] Guard #3253 begins shift
    let begins_regex = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] Guard #(?P<id>\d+) begins shift").unwrap();

    // [1518-04-14 00:51] wakes up
    let wakes_regex = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] wakes up").unwrap();

    // [1518-05-28 00:36] falls asleep
    let sleep_regex = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] falls asleep").unwrap();

    let mut events = Vec::new();
    for line in reader.lines().map(|l| l.expect("Error reading line")) {
        let (t, c) = wakes_regex.captures(line.trim()).map(|c| (Event::Wake, c))
            .or_else(|| sleep_regex.captures(line.trim()).map(|c| (Event::Sleep, c)))
            .or_else(|| begins_regex.captures(line.trim()).map(|c| (Event::Begin, c))).unwrap();
        let time = c["month"].parse::<i32>().unwrap() * 1000000 + c["day"].parse::<i32>().unwrap() * 10000 +
            c["hour"].parse::<i32>().unwrap() * 100 + c["minute"].parse::<i32>().unwrap();
        events.push((time, t,
                     c.name("id").map(|id| id.as_str().parse::<i32>().unwrap()),
                     c.name("minute").map(|m| m.as_str().parse::<i32>().unwrap())));
    }
    events.sort_by_key(|event| event.0);

    let mut time_tables = HashMap::new();
    let mut guard: i32 = -1;
    let mut start = -1;

    for event in events {
        match event {
            (_, Event::Begin, Some(id), _) => {
                guard = id;
            }
            (_, Event::Sleep, _, Some(minute)) => { start = minute; }
            (_, Event::Wake, _, Some(minute)) => {
                let minute = minute;
                let time_table = time_tables.entry(guard).or_insert((0, vec![0; 100]));
                time_table.0 += minute - start;
                for time in start..minute {
                    time_table.1[time as usize] += 1;
                }
            }
            _ => {}
        }
    }

    let entries = time_tables.iter().collect::<Vec<_>>();

    let max = *entries.iter().max_by_key(|entry| (entry.1).0).unwrap();
    let id = max.0;
    let minute = (max.1).1.iter().enumerate().max_by_key(|entry| entry.1).unwrap().0;

    println!("{}", id * minute as i32);

    // Task 2
    let max = entries.iter().max_by_key(|entry| (entry.1).1.iter().max().unwrap()).unwrap();
    let id = max.0;
    let minute = (max.1).1.iter().enumerate().max_by_key(|entry| entry.1).unwrap().0;

    println!("{}", id * minute as i32);

    Ok(())
}
