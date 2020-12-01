use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let value: u64 = line
            .parse()
            .map_err(|err| Error::new(ErrorKind::Other, format!("parse error: {}", err)))?;
        entries.push(value);
    }

    'outer1: for entry1 in &entries {
        for entry2 in &entries {
            if entry1 + entry2 == 2020 {
                println!("Part 1: {}", entry1 * entry2);
                break 'outer1;
            }
        }
    }

    'outer2: for entry1 in &entries {
        for entry2 in &entries {
            for entry3 in &entries {
                if entry1 + entry2 + entry3 == 2020 {
                    println!("Part 2: {}", entry1 * entry2 * entry3);
                    break 'outer2;
                }
            }
        }
    }
    Ok(())
}
