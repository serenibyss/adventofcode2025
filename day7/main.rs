use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use adventofcode2025::aocutils::RunTimer;

struct Manifold {
    manifold: Vec<String>
}

impl Manifold {

    fn new(file: &str) -> io::Result<Manifold> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let manifold = reader.lines()
            .map_while(Result::ok)
            .collect();

        Ok(Manifold { manifold })
    }

    fn start_pos(&self) -> usize {
        self.manifold[0].find("S").unwrap()
    }

    fn calc_beams(&self) -> (usize, Vec<Vec<usize>>) {
        let mut splits: usize = 0;
        let start_pos = self.start_pos();
        let mut beams: Vec<Vec<usize>> = vec![Vec::new(); self.manifold.len()];
        beams[1].push(start_pos);

        for i in 2..self.manifold.len() {
            let line = &self.manifold[i];

            let (a, b) = beams.split_at_mut(i);
            let prev_beam = &a[i-1];
            let curr_beam = &mut b[0];

            for &pos in prev_beam {
                let char = line.chars().nth(pos).unwrap();
                if char == '.' && !curr_beam.contains(&pos) {
                    curr_beam.push(pos);
                } else if char == '^' {
                    splits += 1;
                    let left = pos as i32 - 1;
                    let right = pos as i32 + 1;
                    if left >= 0 && !curr_beam.contains(&(left as usize)) {
                        curr_beam.push(left as usize);
                    }
                    if (right as usize) < line.len() && !curr_beam.contains(&(right as usize)) {
                        curr_beam.push(right as usize);
                    }
                }
            }
        }

        (splits, beams)
    }
}

fn main() -> io::Result<()> {
    let timer = RunTimer::new();

    let manifold = Manifold::new("day7/input.txt")?;
    let (splits, _) = manifold.calc_beams();
    println!("Splits: {}", splits);

    timer.finish();

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_start_pos() {
        let manifold = Manifold::new("day7/testdata/input_part_1.txt").unwrap();
        assert_eq!(manifold.start_pos(), 7);
    }

    #[test]
    fn test_calc_beams_splits() {
        let manifold = Manifold::new("day7/testdata/input_part_1.txt").unwrap();
        let (splits, _) = manifold.calc_beams();
        assert_eq!(splits, 21);
    }
}
