use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io;

fn main() -> io::Result<()> {
    let f = File::open("../input1.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    let mut frequency: i32 = 0;

    while reader.read_line(&mut buffer)? > 0 {
        let number = buffer.trim().parse::<i32>();
        if let Ok(number) = number {
            frequency += number;
        } else if let Err(e) = number {
            println!("Error: {}, '{}'", e, buffer);
        }
        buffer = String::new();
    }

    println!("Result {}", frequency);

    Ok(())
}
