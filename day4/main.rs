use std::{fs, io};
use std::io::BufRead;

fn main() -> io::Result<()> {
    let raw = fs::read_to_string("day4/input.txt")?;
    let mut grid = to_grid(raw);

    let mut total: u64 = 0;
    loop {
        let positions = get_positions(&grid);
        if positions.len() == 0 {
            break;
        }
        total += positions.len() as u64;

        for &(r, c) in positions.as_slice() {
            if r < grid.len() && c < grid[r].len() {
                grid[r][c] = '.';
            }
        }
    }

    println!("Total: {}", total);

    Ok(())
}

fn to_grid(raw: String) -> Vec<Vec<char>> {
    raw.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn check_position(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if grid[row][col] != '@' {
        return false;
    }

    let mut papers = 0;
    const DIRS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1)
    ];

    for (r, c) in DIRS {
        let nr = r + row as isize;
        let nc = c + col as isize;

        if nr < 0 || nc < 0 { continue; }
        let (nr, nc) = (nr as usize, nc as usize);
        if nr >= grid.len() || nc >= grid[nr].len() { continue; }

        if grid[nr][nc] == '@' {
            papers += 1;
        }
    }

    papers < 4
}

fn get_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if check_position(&grid, row, col) {
                positions.push((row, col));
            }
        }
    }

    positions
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_position() -> io::Result<()> {
        let raw = fs::read_to_string("day4/testdata/input_part_1.txt")?;
        let grid = to_grid(raw);
        let mut positions: u64 = 0;

        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if check_position(&grid, row, col) {
                    positions += 1;
                }
            }
        }
        assert_eq!(positions, 13);
        Ok(())
    }
}
