use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut sum = 0;

    let input = File::open("day3/input.txt")?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line: String = line?;
        sum += find_highest_subnumber(line.as_str());
    }

    println!("Sum: {}", sum);

    Ok(())
}

fn find_highest_subnumber(raw: &str) -> i32 {
    let mut highest = 0;


    for (i, c) in raw.chars().enumerate() {
        for c2 in raw.chars().skip(i + 1) {
            let test_str = format!("{}{}", c, c2);
            let test_num: i32 = test_str.as_str().parse().unwrap();

            if test_num > highest {
                highest = test_num;
            }
        }
    }

    highest
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_highest_subnumber() {
        assert_eq!(find_highest_subnumber("72111111111"), 72);
        assert_eq!(find_highest_subnumber("99000000000"), 99);
        assert_eq!(find_highest_subnumber("76543217699"), 99);
        assert_eq!(find_highest_subnumber("90000000009"), 99);
    }
}
