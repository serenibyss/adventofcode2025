use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

struct FreshDB {
    fresh_ranges: Vec<(u64, u64)>,
    test_values: Vec<u64>,
}

impl FreshDB {
    fn new(db_file: &str) -> io::Result<FreshDB> {
        let file = File::open(db_file)?;
        let reader = BufReader::new(file);

        let mut ranges = Vec::new();
        let mut values = Vec::new();

        let mut in_values = false;

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            if line.is_empty() {
                in_values = true;
                continue;
            }

            if !in_values {
                let (start, end) = line
                    .split_once('-')
                    .unwrap();

                let start: u64 = start.parse().unwrap();
                let end: u64 = end.parse().unwrap();
                ranges.push((start, end));
            } else {
                let value: u64 = line.parse().unwrap();
                values.push(value);
            }
        }

        Ok(Self {
            fresh_ranges: ranges,
            test_values: values,
        })
    }

    fn test_freshness(&self) -> u64 {
        let mut fresh_count: u64 = 0;
        for &value in &self.test_values {
            let mut was_fresh = false;
            for &(start, end) in &self.fresh_ranges {
                if value >= start && value <= end {
                    was_fresh = true;
                    continue;
                }
            }
            if was_fresh {
                fresh_count += 1;
                continue;
            }
        }

        fresh_count
    }
}

fn main() -> io::Result<()> {
    let db = FreshDB::new("day5/input.txt")?;
    println!("Fresh count: {}", db.test_freshness());
    Ok(())
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {
        let db = FreshDB::new("day5/testdata/input_part_1.txt").unwrap();
        assert_eq!(db.test_freshness(), 3);
    }
}
