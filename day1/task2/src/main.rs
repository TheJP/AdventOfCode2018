use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;
use std::collections::HashSet;
use std::io::Seek;
use std::io::SeekFrom;

fn main() -> io::Result<()> {
    let f = File::open("../input2.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut frequency: i32 = 0;
    let mut frequencies = HashSet::new();
    frequencies.insert(0);

    loop {
        while reader.read_line(&mut buffer)? > 0 {
            let number = buffer.trim().parse::<i32>();
            if let Ok(number) = number {
                frequency += number;
            } else if let Err(e) = number {
                println!("Error: {}, '{}'", e, buffer);
            }

            if frequencies.contains(&frequency) {
                println!("Result {}", frequency);
                return Ok(());
            } else {
                frequencies.insert(frequency);
                println!("{}", frequency);
            }

            buffer = String::new();
        }

        reader.seek(SeekFrom::Start(0))?;
    }
}
