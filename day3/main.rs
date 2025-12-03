use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()> {
    let mut sum: u64 = 0;

    let input = File::open("day3/input.txt")?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line: String = line?;
        sum += find_highest_subnumber_part_2(line.as_str());
    }

    println!("Sum: {}", sum);

    Ok(())
}

#[allow(dead_code)]
fn find_highest_subnumber_part_1(raw: &str) -> u64 {
    let mut highest: u64 = 0;


    for (i, c) in raw.chars().enumerate() {
        for c2 in raw.chars().skip(i + 1) {
            let test_str = format!("{}{}", c, c2);
            let test_num: u64 = test_str.as_str().parse().unwrap();

            if test_num > highest {
                highest = test_num;
            }
        }
    }

    highest
}

#[allow(dead_code)]
fn find_highest_subnumber_part_2(raw: &str) -> u64 {
    let mut str = raw.to_string();

    while str.len() > 12 {
        str = remove_worst_digit(str);
    }

    str.as_str().parse::<u64>().unwrap()
}

fn remove_worst_digit(raw: String) -> String {
    let bytes = raw.as_bytes();

    for i in 0..raw.len()-1 {
        let c = bytes[i];
        let c1 = bytes[i + 1];

        if c < c1 {
            return format!("{}{}", raw[..i].to_owned(), raw[i+1..].to_owned()).to_owned();
        }
    }

    raw[..raw.len()-1].to_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_highest_subnumber_part_1() {
        assert_eq!(find_highest_subnumber_part_1("72111111111"), 72);
        assert_eq!(find_highest_subnumber_part_1("99000000000"), 99);
        assert_eq!(find_highest_subnumber_part_1("76543217699"), 99);
        assert_eq!(find_highest_subnumber_part_1("90000000009"), 99);
    }

    #[test]
    fn test_find_highest_subnumber_part_2() {
        assert_eq!(find_highest_subnumber_part_2("987654321111111"), 987654321111);
        assert_eq!(find_highest_subnumber_part_2("811111111111119"), 811111111119);
        assert_eq!(find_highest_subnumber_part_2("234234234234278"), 434234234278);
        assert_eq!(find_highest_subnumber_part_2("818181911112111"), 888911112111);
    }

    #[test]
    fn test_remove_worst_digit() {
        assert_eq!(remove_worst_digit("987654321111111".to_owned()), "98765432111111");
        assert_eq!(remove_worst_digit("98765432111111".to_owned()), "9876543211111");
        assert_eq!(remove_worst_digit("9876543211111".to_owned()), "987654321111");

        assert_eq!(remove_worst_digit("811111111111119".to_owned()), "81111111111119");
        assert_eq!(remove_worst_digit("81111111111119".to_owned()), "8111111111119");
        assert_eq!(remove_worst_digit("8111111111119".to_owned()), "811111111119");

        assert_eq!(remove_worst_digit("234234234234278".to_owned()), "34234234234278");
        assert_eq!(remove_worst_digit("34234234234278".to_owned()), "4234234234278");
        assert_eq!(remove_worst_digit("4234234234278".to_owned()), "434234234278");

        assert_eq!(remove_worst_digit("818181911112111".to_owned()), "88181911112111");
        assert_eq!(remove_worst_digit("88181911112111".to_owned()), "8881911112111");
        assert_eq!(remove_worst_digit("8881911112111".to_owned()), "888911112111");
    }
}
