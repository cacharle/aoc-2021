use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "11/input.txt";

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
                .ok_or("cannot parse character as digit")?,
        ))
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            write!(f, "{}: ", i)?;
            for j in 0..self.0[0].len() {
                write!(f, "{} ", self.0[i][j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn step(&mut self) -> usize {
        // Throw up for me please
        let mut nines_pos = HashSet::new();
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.0[i][j] == 9 {
                    nines_pos.insert((i, j));
                }
            }
        }
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                self.0[i][j] += 1;
            }
        }
        let mut flashes = 0;
        let mut visited_flashed = HashSet::new();
        while self.0.iter().any(|r| r.iter().any(|&x| x > 9))
            || visited_flashed.len() < nines_pos.len()
        {
            for i in 0..self.0.len() {
                for j in 0..self.0[0].len() {
                    if (self.0[i][j] > 9 || nines_pos.contains(&(i, j)))
                        && !visited_flashed.contains(&(i, j))
                    {
                        visited_flashed.insert((i, j));
                        self.0[i][j] = 0;
                        flashes += 1;
                        let modifiers = [
                            (1, 0),
                            (-1, 0),
                            (0, 1),
                            (0, -1),
                            (1, 1),
                            (-1, -1),
                            (1, -1),
                            (-1, 1),
                        ];
                        for m in modifiers {
                            let ni: i32 = (i as i32) + m.0;
                            let nj: i32 = (j as i32) + m.1;
                            let width = self.0[0].len() as i32;
                            let height = self.0.len() as i32;
                            if ni >= 0
                                && nj >= 0
                                && ni < height
                                && nj < width
                                && !visited_flashed.contains(&(ni as usize, nj as usize))
                            {
                                self.0[ni as usize][nj as usize] += 1;
                            }
                        }
                    }
                }
            }
        }
        flashes
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let mut grid: Grid = file_content.parse()?;
    let mut s = 0;
    for _ in 0..100 {
        s += grid.step();
    }
    println!("part1 result: {}", s);

    let mut grid: Grid = file_content.parse()?;
    for i in 1.. {
        if grid.step() == grid.0.len() * grid.0[0].len() {
            println!("part2 result: {}", i);
            break;
        }
    }
    Ok(())
}
