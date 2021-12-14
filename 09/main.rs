use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "09/input.txt";

struct Grid(Vec<Vec<u32>>);

impl FromStr for Grid {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid(
            s.lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10))
                        .collect::<Option<Vec<_>>>()
                })
                .collect::<Option<Vec<_>>>()
                .ok_or("could not convert to int")?,
        ))
    }
}

impl Grid {
    fn neighbours_pos(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let modifiers = [[1, 0], [-1, 0], [0, 1], [0, -1]];
        let mut neighbours = Vec::new();
        for m in modifiers {
            let y = (y as i32) + m[0];
            let x = (x as i32) + m[1];
            if y >= 0 && x >= 0 && y < self.0.len() as i32 && x < self.0[0].len() as i32 {
                neighbours.push((y as usize, x as usize));
            }
        }
        neighbours
    }

    fn positions_to_values(&self, pos: Vec<(usize, usize)>) -> Vec<u32> {
        pos.iter().map(|(y, x)| self.0[*y][*x]).collect()
    }

    fn low_points_pos(&self) -> Vec<(usize, usize)> {
        let mut low_points = Vec::new();
        for y in 0..self.0.len() {
            for x in 0..self.0[0].len() {
                let point = self.0[y][x];
                let neighbours = self.positions_to_values(self.neighbours_pos(y, x));
                if neighbours.iter().all(|p| p > &point) {
                    low_points.push((y, x));
                }
            }
        }
        low_points
    }

    fn risk_levels_sum(&self) -> u32 {
        self.positions_to_values(self.low_points_pos())
            .iter()
            .map(|p| p + 1)
            .sum()
    }

    fn basin(&self, y: usize, x: usize) -> Vec<u32> {
        let mut visited = HashSet::new();
        self.basin_rec(y, x, &mut visited)
    }

    fn basin_rec(&self, y: usize, x: usize, visited: &mut HashSet<(usize, usize)>) -> Vec<u32> {
        if visited.contains(&(y, x)) {
            return vec![];
        }
        let mut values = vec![self.0[y][x]];
        visited.insert((y, x));
        for (ny, nx) in self.neighbours_pos(y, x) {
            if self.0[ny][nx] > self.0[y][x] && self.0[ny][nx] != 9 {
                values.extend(self.basin_rec(ny, nx, visited));
            }
        }
        values
    }

    fn basin_sizes(&self) -> Vec<usize> {
        self.low_points_pos()
            .iter()
            .map(|(y, x)| self.basin(*y, *x).len())
            .collect()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let grid: Grid = file_content.parse()?;
    println!("part1 result: {}", grid.risk_levels_sum());

    let mut sizes = grid.basin_sizes();
    sizes.sort();
    let result: usize = sizes[sizes.len() - 3..].iter().product();
    println!("part2 result: {}", result);

    Ok(())
}
