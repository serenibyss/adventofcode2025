use std::{fs, io};
use adventofcode2025::aocutils::{CondRev, RunTimer};

struct MathTable {
    numbers: Vec<Vec<i64>>,
    symbols: Vec<char>,
}

impl MathTable {

    fn new(input: &str, reversed: bool) -> io::Result<MathTable> {
        let raw = fs::read_to_string(input)?;
        let mut lines = raw.lines().collect::<Vec<_>>();

        Ok(MathTable {
            symbols: lines.pop()
                .unwrap()
                .split_whitespace()
                .map(|s| s.chars().next().unwrap())
                .cond_rev(reversed)
                .collect(),
            numbers: match reversed {
                true => MathTable::read_data_reversed(lines),
                false => MathTable::read_data(lines),
            },
        })
    }

    fn read_data(lines: Vec<&str>) -> Vec<Vec<i64>> {
        let rows: Vec<Vec<i64>> = lines.iter()
            .map(|line| line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>())
            .collect();

        rows.iter().fold(
            vec![Vec::with_capacity(rows.len()); rows[0].len()],
            |mut acc, row| {
                (0..rows[0].len()).for_each(|c| acc[c].push(row[c]));
                acc
            })
    }

    fn read_data_reversed(lines: Vec<&str>) -> Vec<Vec<i64>> {
        (0..lines[0].len())
            .rev()
            .map(|i| lines.iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>())
            .fold(Vec::new(), |mut acc: Vec<Vec<i64>>, s| {
                if s.trim().is_empty() {
                    acc.push(Vec::new());
                } else {
                    if acc.is_empty() {
                        acc.push(Vec::new());
                    }
                    acc.last_mut()
                        .unwrap()
                        .push(s.trim()
                            .parse::<i64>()
                            .unwrap());
                }
                acc
            })
    }

    fn size(&self) -> usize {
        self.symbols.len()
    }

    fn sum(&self) -> i64 {
        (0..self.size())
            .map(|i| (&self.numbers[i], self.symbols[i]))
            .map(|(numbers, symbol)| numbers.iter()
                .copied()
                .reduce(|acc, n| match symbol {
                    '+' => acc + n,
                    '*' => acc * n,
                    _ => unreachable!(),
                })
                .unwrap_or(0))
            .sum()
    }
}

fn main() -> io::Result<()> {
    let timer = RunTimer::new();

    let table = MathTable::new("day6/input.txt", false)?;
    println!("Standard Sum: {}", table.sum());

    let table = MathTable::new("day6/input.txt", true)?;
    println!("Columnar Sum: {}", table.sum());

    timer.finish();

    Ok(())
}

#[cfg(test)]
mod tests {

    use adventofcode2025::aocutils::cmp_vec;
    use super::*;

    #[test]
    fn test_standard_parse() {
        let table = MathTable::new("day6/testdata/input_part_1.txt", false).unwrap();
        assert_eq!(table.size(), 4);
        assert!(cmp_vec(&table.numbers[0], &vec![123, 45, 6]));
        assert!(cmp_vec(&table.numbers[1], &vec![328, 64, 98]));
        assert!(cmp_vec(&table.numbers[2], &vec![51, 387, 215]));
        assert!(cmp_vec(&table.numbers[3], &vec![64, 23, 314]));
    }

    #[test]
    fn test_standard_sum() {
        let table = MathTable::new("day6/testdata/input_part_1.txt", false).unwrap();
        assert_eq!(table.sum(), 4277556);
    }

    #[test]
    fn test_reversed_parse() {
        let table = MathTable::new("day6/testdata/input_part_1.txt", true).unwrap();
        assert_eq!(table.size(), 4);
        assert!(cmp_vec(&table.numbers[0], &vec![4, 431, 623]));
        assert!(cmp_vec(&table.numbers[1], &vec![175, 581, 32]));
        assert!(cmp_vec(&table.numbers[2], &vec![8, 248, 369]));
        assert!(cmp_vec(&table.numbers[3], &vec![356, 24, 1]));
    }

    #[test]
    fn test_sum_reversed() {
        let table = MathTable::new("day6/testdata/input_part_1.txt", true).unwrap();
        assert_eq!(table.sum(), 3263827);
    }
}
