use std::{fs, io};

struct MathTable {
    numbers: Vec<Vec<i64>>,
    symbols: Vec<char>,
    size: usize,
}

impl MathTable {

    fn new(input: &str) -> io::Result<MathTable> {
        let raw = fs::read_to_string(input)?;
        let mut lines: Vec<&str> = raw.lines().collect();

        let symbol_line = lines.pop().unwrap();

        let rows: Vec<Vec<i64>> = lines.iter()
            .map(|line| line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>())
            .collect();

        let row_count = rows.len();
        let col_count = rows[0].len();

        let mut numbers = vec![Vec::with_capacity(row_count); col_count];
        for r in 0..row_count {
            for c in 0..col_count {
                numbers[c].push(rows[r][c]);
            }
        }

        let symbols: Vec<char> = symbol_line.split_whitespace()
            .map(|s| s.chars().next().unwrap())
            .collect();
        let size = symbols.len();

        Ok(MathTable { numbers, symbols, size })
    }

    fn sum_all_formulas(&self) -> i64 {
        (0..self.size).into_iter()
            .map(|i| (&self.numbers[i], self.symbols[i]))
            .map(|(numbers, symbol)| numbers.iter()
                .copied()
                .reduce(|acc, n| match symbol {
                    '+' => acc + n,
                    '*' => acc * n,
                    _ => unreachable!(),
                })
                .unwrap_or(0i64))
            .sum()
    }
}

fn main() -> io::Result<()> {
    let table = MathTable::new("day6/input.txt")?;
    println!("Sum: {}", table.sum_all_formulas());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum_all_formulas() {
        let table = MathTable::new("day6/testdata/input_part_1.txt").unwrap();
        assert_eq!(table.sum_all_formulas(), 4277556);
    }
}
