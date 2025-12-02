use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut invalid_sum: u64 = 0;

    let input = File::open("day2/input.txt")?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line: String = line?;

        for id in line.as_str().split(',') {
            let (left, right) = id.split_once('-').unwrap();

            check_range(left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap(), &mut invalid_sum);
        }
    }

    println!("Invalid ID sum: {}", invalid_sum);

    Ok(())
}

fn check_range(start: u64, end: u64, sum: &mut u64) {
    for i in start..=end {
        if is_invalid_id(i) {
            *sum += i;
        }
    }
}

fn is_invalid_id(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if len % 2 != 0 {
        return false
    }
    let half = len / 2;
    &id_str[..half] == &id_str[half..]
}

#[cfg(test)]
mod tests {
    use super::is_invalid_id;

    #[test]
    fn test_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(!is_invalid_id(12));

        assert!(!is_invalid_id(111));
        assert!(!is_invalid_id(112));

        assert!(is_invalid_id(112112));
        assert!(!is_invalid_id(1121122));
    }
}
