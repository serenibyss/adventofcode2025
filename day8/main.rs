use std::cmp::{min, Ordering};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::mem::take;
use std::rc::Rc;
use adventofcode2025::aocutils::RunTimer;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }

    fn dist(&self, other: &Position) -> f32 {
        let xs = ((self.x - other.x) as i64).pow(2);
        let ys = ((self.y - other.y) as i64).pow(2);
        let zs = ((self.z - other.z) as i64).pow(2);
        ((xs + ys + zs) as f64).sqrt() as f32
    }

    fn mult_x(&self, other: &Position) -> u64 {
        self.x as u64 * other.x as u64
    }
}

#[derive(Debug, Clone)]
struct Link {
    pos1: Rc<Position>,
    pos2: Rc<Position>,
    dist: f32
}

impl Link {
    fn new(pos1: Rc<Position>, pos2: Rc<Position>) -> Link {
        let dist = pos1.dist(&pos2);
        Link { pos1, pos2, dist }
    }

    fn mult_x(&self) -> u64 {
        self.pos1.as_ref().mult_x(self.pos2.as_ref())
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        self.dist == other.dist
    }
}

impl PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

#[derive(Debug, Clone, Default)]
struct Circuit {
    positions: Vec<Position>
}

impl Circuit {
    fn new(initial: Position) -> Circuit {
        Circuit {
            positions: vec![initial]
        }
    }

    fn connects(&self, link: &Link) -> bool {
        for l in &self.positions {
            if l == link.pos1.as_ref() {
                return true;
            }
            if l == link.pos2.as_ref() {
                return true
            }
        }

        false
    }

    fn connect(&mut self, link: &Link) {
        self.connect_pos(link.pos1.as_ref());
        self.connect_pos(link.pos2.as_ref());
    }

    fn connect_pos(&mut self, position: &Position) {
        if !self.positions.contains(position) {
            self.positions.push(*position);
        }
    }

    fn merge(&mut self, other: &Circuit) -> &Self {
        for position in other.positions.iter() {
            self.connect_pos(position);
        }
        self
    }

    fn size(&self) -> usize {
        self.positions.len()
    }
}

impl PartialEq for Circuit {
    fn eq(&self, other: &Circuit) -> bool {
        self.size() == other.size()
    }
}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.size().partial_cmp(&other.size())
    }
}

struct Calculator {
    positions: Vec<Position>,
    links: Vec<Link>
}

impl Calculator {
    fn new(file: &str, max_connections: usize) -> io::Result<Calculator> {
        let file = File::open(file)?;
        let lines = BufReader::new(file).lines();
        let mut positions = Vec::new();

        // Parse all positions
        for line in lines {
            let line = line?;
            let pos_raw: Vec<&str> = line.trim().split(',').collect();
            let pos = Position::new(
                pos_raw[0].parse().unwrap(),
                pos_raw[1].parse().unwrap(),
                pos_raw[2].parse().unwrap(),
            );
            positions.push(Rc::new(pos));
        }

        // Calculate all links
        let mut links: Vec<Link> = Vec::new();
        for i in 0..positions.len()-1 {
            for j in i+1..positions.len() {
                links.push(Link::new(positions[i].clone(), positions[j].clone()));
            }
        }

        // Sort links by distance
        links.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Limit links to specified number
        let links = links[0..min(max_connections, links.len())].to_vec();

        // Calculate all positions we start with
        let mut circuit = Circuit::default();
        for link in links.iter() {
            circuit.connect(link);
        }
        let positions = circuit.positions;

        Ok(Calculator { positions, links })
    }

    fn calculate_circuits(&self) -> (Vec<Circuit>, u64) {
        let mut unify_point: u64 = 0;
        let mut circuits: Vec<Circuit> = self.positions.iter()
            .copied()
            .map(Circuit::new)
            .collect();

        for link in self.links.iter() {
            let mut matches = Vec::new();

            for (i, circuit) in circuits.iter().enumerate() {
                if circuit.connects(link) {
                    matches.push(i);
                }
            }

            if !matches.is_empty() {
                let first = matches[0];
                circuits[first].connect(link);
                for &i in matches.iter().skip(1) {
                    let other = take(&mut circuits[i]);
                    circuits[first].merge(&other);
                    circuits.remove(i);
                }
            }

            if circuits.len() == 1 {
                unify_point = link.mult_x();
                break;
            }
        }

        circuits.sort_by(|a, b| b.partial_cmp(a).unwrap());

        (circuits, unify_point)
    }
}

fn main() -> io::Result<()> {
    let mut timer = RunTimer::new();

    let calculator = Calculator::new("day8/input.txt", 1000)?;
    let (circuits, _) = calculator.calculate_circuits();
    let size = circuits[0].size() * circuits[1].size() * circuits[2].size();
    println!("Largest 3 circuits multiplied (1000): {}", size);
    timer.mark();

    let calculator = Calculator::new("day8/input.txt", usize::MAX)?;
    let (_, unify_point) = calculator.calculate_circuits();
    println!("Unify point (max): {}", unify_point);
    timer.finish();

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_dist() {
        let pos1 = Position::new(162, 817, 812);
        let pos2 = Position::new(425, 690, 689);
        assert_eq!(pos1.dist(&pos2), 316.9022);
    }

    #[test]
    fn test_shortest_distance() {
        let calculator = Calculator::new("day8/testdata/input_part_1.txt", 1).unwrap();

        let pos1 = Position::new(162, 817, 812);
        let pos2 = Position::new(425, 690, 689);
        let link = Link::new(Rc::new(pos1), Rc::new(pos2));
        assert_eq!(calculator.links[0], link);
    }

    #[test]
    fn test_calculate_circuits() {
        let calculator = Calculator::new("day8/testdata/input_part_1.txt", 10).unwrap();
        let (circuits, _) = calculator.calculate_circuits();

        let size0 = circuits[0].size();
        let size1 = circuits[1].size();
        let size2 = circuits[2].size();
        let size = size0 * size1 * size2;
        assert_eq!(size0, 5);
        assert_eq!(size1, 4);
        assert_eq!(size2, 2);
        assert_eq!(size, 40);
    }

    #[test]
    fn test_calculate_unify_point() {
        let calculator = Calculator::new("day8/testdata/input_part_1.txt", usize::MAX).unwrap();
        let (_, unify_point) = calculator.calculate_circuits();
        assert_eq!(unify_point, 25272);
    }
}
