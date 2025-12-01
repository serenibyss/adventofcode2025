use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

fn main() -> io::Result<()> {
    let mut num_zero: u32 = 0;
    let mut dial: i32 = 50;

    let input = File::open("day1/input.txt")?;
    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line?;

        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let amount: i32 = chars.as_str().parse()
            .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))?;

        let left = direction == 'L';

        rotate(left, amount, &mut dial, &mut num_zero);
    }

    println!("Num Zeroes: {}", num_zero);

    Ok(())
}

fn rotate(left: bool, amount: i32, dial: &mut i32, num_zero: &mut u32) {
    let mut amount = amount;

    while amount > 0 {
        if left {
            if *dial == 0 {
                *dial = 99;
            } else {
                *dial -= 1;
            }
        } else {
            if *dial == 99 {
                *dial = 0
            } else {
                *dial += 1;
            }
        }

        if *dial == 0 {
            *num_zero += 1;
        }

        amount -= 1;
    }
}
