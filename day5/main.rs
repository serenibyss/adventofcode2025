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
        let ranges = &self.fresh_ranges;
        let values = &self.test_values;
        values.iter()
            .filter(|&&value| ranges.iter()
                .any(|&(start, end)| value >= start && value <= end))
            .count() as u64
    }

    fn collapse_ranges(&mut self) {
        let ranges = &mut self.fresh_ranges;
        ranges.sort_by_key(|r| r.0);

        let mut collapsed: Vec<(u64, u64)> = Vec::new();
        let mut current = ranges[0];

        for &(start, end) in ranges.iter().skip(1) {
            if start <= current.1 {
                current.1 = current.1.max(end);
            } else {
                collapsed.push(current);
                current = (start, end);
            }
        }

        collapsed.push(current);
        collapsed.clone_into(ranges);
    }

    fn count_total(&self) -> u64 {
        let mut total: u64 = 0;

        for &(start, end) in &self.fresh_ranges {
            total += end - start + 1;
        }

        total
    }
}

fn main() -> io::Result<()> {
    let mut db = FreshDB::new("day5/input.txt")?;
    println!("Fresh count: {}", db.test_freshness());
    db.collapse_ranges();
    println!("Fresh ID count: {}", db.count_total());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_test_freshness() {
        let db = FreshDB::new("day5/testdata/input_part_1.txt").unwrap();
        assert_eq!(db.test_freshness(), 3);
    }

    #[test]
    #[should_panic]
    fn test_collapse_ranges() {
        let mut db = FreshDB::new("day5/testdata/input_part_1.txt").unwrap();
        db.collapse_ranges();
        // 3-5 10-20
        println!("{:?}", db.fresh_ranges);
        assert_eq!(db.fresh_ranges[0], (3u64, 5u64));
        assert_eq!(db.fresh_ranges[1], (10u64, 20u64));
        assert!(db.fresh_ranges[2] == (0u64, 0u64), "Panicked!");
    }

    #[test]
    fn test_count_total() {
        let mut db = FreshDB::new("day5/testdata/input_part_1.txt").unwrap();
        db.collapse_ranges();
        assert_eq!(db.count_total(), 14);
    }
}
