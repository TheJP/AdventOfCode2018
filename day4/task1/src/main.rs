extern crate regex;

use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;

enum Event {
    Wake,
    Sleep,
    Begin,
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // [1518-08-23 23:56] Guard #3253 begins shift
    let begins_regex = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] Guard #(?P<id>\d+) begins shift").unwrap();

    // [1518-04-14 00:51] wakes up
    let wakes_regex = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] wakes up").unwrap();

    // [1518-05-28 00:36] falls asleep
    let sleep_regex = Regex::new(
        r"\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] falls asleep").unwrap();


    let mut time_table = Vec::new();
    let mut time_tables = HashMap::new();

    let mut events = reader.lines()
        .map(|l| l.expect("Error while reading lines"))
        .map(|line| wakes_regex.captures(line.trim()).map(|c| (Event::Wake, c))
            .or_else(|| sleep_regex.captures(line.trim()).map(|c| (Event::Sleep, c)))
            .or_else(|| begins_regex.captures(line.trim()).map(|c| (Event::Begin, c))).unwrap())
        .map(|(t, c)| (c["month"].parse::<i32>().unwrap() * 1000000 + c["day"].parse::<i32>().unwrap() * 10000 + c["hour"].parse::<i32>().unwrap() * 100 + c["minute"].parse::<i32>().unwrap(), t, c))
        .collect::<Vec<_>>();
    events.sort_by_key(|&event| event.0);

//        match event {
//            (_, Event::Begin, c) => {
//                let guard = c["id"].parse().unwrap();
//                time_table = time_tables.entry(guard).or_insert(Vec::new())
//            }
//            (day, t, _) => { time_table.push((day, t)); }
//}

    println!("");

    Ok(())
}
