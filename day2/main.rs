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
        // let invalid = is_invalid_id_part_1(i);
        let invalid = is_invalid_id_part_2(i);

        if invalid {
             *sum += i;
        }
    }
}

fn is_invalid_id_part_1(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    if len % 2 != 0 {
        return false
    }
    let half = len / 2;
    &id_str[..half] == &id_str[half..]
}

fn is_invalid_id_part_2(id: u64) -> bool {
    let id_str = id.to_string();
    let len = id_str.len();

    for sub_len in 1..=len/2 {
        if len % sub_len != 0 {
            continue;
        }

        let pattern = &id_str[..sub_len];

        let mut i = sub_len;
        while i < len {
            if &id_str[i..i + sub_len] != pattern {
                break;
            }
            i += sub_len;
        }

        if i == len {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id_part_1() {
        assert!(is_invalid_id_part_1(11));
        assert!(!is_invalid_id_part_1(12));

        assert!(!is_invalid_id_part_1(111));
        assert!(!is_invalid_id_part_1(112));

        assert!(is_invalid_id_part_1(112112));
        assert!(!is_invalid_id_part_1(1121122));
    }

    #[test]
    fn test_invalid_id_part_2() {
        assert!(is_invalid_id_part_2(11));
        assert!(!is_invalid_id_part_2(12));

        assert!(is_invalid_id_part_2(111));
        assert!(!is_invalid_id_part_2(112));

        assert!(is_invalid_id_part_2(112112));
        assert!(!is_invalid_id_part_2(1121122));

        assert!(is_invalid_id_part_2(11111111111));
        assert!(is_invalid_id_part_2(1212121212));
        assert!(is_invalid_id_part_2(123412341234));
    }
}
