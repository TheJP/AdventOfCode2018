use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

/// Returns the position at which two strings differ, if they have exactly one difference.
///
/// # Examples
///
/// ```
/// assert_eq!(differ_by_one(&"abc".to_string(), &"abz".to_string()), Some(2));
/// assert_eq!(differ_by_one(&"abc".to_string(), &"abc".to_string()), None);
/// assert_eq!(differ_by_one(&"abc".to_string(), &"xyz".to_string()), None);
/// ```
fn differ_by_one(a: &String, b: &String) -> Option<usize> {
    let indexed = a.chars().enumerate().zip(b.chars());
    let mut changes = indexed.filter(|&((_, letter_a), letter_b)| letter_a != letter_b);
    let solution = changes.next();
    if changes.next() == None { solution.map(|((i, _), _)| i) } else { None }
}

fn main() -> io::Result<()> {
    let f = File::open("../input.txt")?;
    let reader = BufReader::new(f);

    let mut lines = Vec::new();

    for line in reader.lines().map(|l| l.expect("Error while reading lines")) {
        for old_line in &lines {
            match differ_by_one(&line, old_line) {
                Some(position) => {
                    println!("{}{}",
                             line.get({ ..position }).unwrap_or(""),
                             line.get({ (position + 1).. }).unwrap_or(""));
                    return Ok(());
                }
                None => {}
            }
        }
        lines.push(line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_differ_by_one_1() {
        assert_eq!(differ_by_one(&"abc".to_string(), &"abz".to_string()), Some(2));
        assert_eq!(differ_by_one(&"abcefgh".to_string(), &"abczfgh".to_string()), Some(3));
        assert_eq!(differ_by_one(&"abcefgh".to_string(), &"zbcefgh".to_string()), Some(0));
        assert_eq!(differ_by_one(&"abcefgh".to_string(), &"abcefgz".to_string()), Some(6));
    }

    #[test]
    fn test_differ_by_one_2() {
        assert_eq!(differ_by_one(&"abc".to_string(), &"abc".to_string()), None);
    }

    #[test]
    fn test_differ_by_one_3() {
        assert_eq!(differ_by_one(&"abc".to_string(), &"xyz".to_string()), None);
        assert_eq!(differ_by_one(&"abcefgh".to_string(), &"zbcefgz".to_string()), None);
    }
}

