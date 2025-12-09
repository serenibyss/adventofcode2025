use std::cmp::{max, min, Ordering};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use adventofcode2025::aocutils::RunTimer;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x, y }
    }

    fn area_with(&self, other: &Point) -> u64 {
        let min_x = min(self.x, other.x);
        let max_x = max(self.x, other.x);
        let min_y = min(self.y, other.y);
        let max_y = max(self.y, other.y);

        (max_x - min_x + 1) as u64 * (max_y - min_y + 1) as u64
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.y < other.y {
            Some(Ordering::Less)
        } else if self.y > other.y {
            Some(Ordering::Greater)
        } else if self.x < other.x {
                Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

struct Grid {
    positions: Vec<Point>
}

impl Grid {
    fn new(file: &str) -> io::Result<Grid> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let mut positions = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let (x, y) = line.split_once(',').unwrap();
            positions.push(Point::new(x.parse().unwrap(), y.parse().unwrap()));
        }

        positions.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Ok(Grid { positions })
    }

    fn max_area(&self) -> u64 {
        let mut max_area = 0;
        for i in 0..self.positions.len()-1 {
            for j in i+1..self.positions.len() {
                let p1 = &self.positions[i];
                let p2 = &self.positions[j];
                let area = p1.area_with(p2);
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area
    }
}

fn main() -> io::Result<()> {
    let mut timer = RunTimer::new();

    let grid = Grid::new("day9/input.txt")?;
    println!("Largest area: {}", grid.max_area());
    timer.finish();

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_max_area() {
        let grid = Grid::new("day9/testdata/input_part_1.txt").unwrap();
        assert_eq!(grid.max_area(), 50);
    }
}
