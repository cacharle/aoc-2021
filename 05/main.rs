use std::fs;
use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;

const INPUT_FILENAME: &str = "05/input.txt";

struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Segment {
    fn direction(&self) -> Direction {
        use Direction::*;
        if self.y1 == self.y2 {
            Horizontal
        } else if self.x1 == self.x2 {
            Vertical
        } else {
            Diagonal
        }
    }

    fn positions(&self) -> Vec<(i32, i32)> {
        // SPAGHETTI CODE, HERE I COME
        use Direction::*;
        let (x_mod, y_mod) = match self.direction() {
            Diagonal => {
                if self.x1 < self.x2 && self.y1 < self.y2 {
                    (1, 1)
                } else if self.x1 > self.x2 && self.y1 < self.y2 {
                    (-1, 1)
                } else if self.x1 < self.x2 && self.y1 > self.y2 {
                    (1, -1)
                } else if self.x1 > self.x2 && self.y1 > self.y2 {
                    (-1, -1)
                } else {
                    panic!("yo");
                }
            }
            Horizontal => {
                if self.x1 < self.x2 {
                    (1, 0)
                } else {
                    (-1, 0)
                }
            },
            Vertical => {
                if self.y1 < self.y2 {
                    (0, 1)
                } else {
                    (0, -1)
                }
            }
        };
        let (mut x, mut y) = (self.x1, self.y1);
        let mut ret = Vec::new();
        while (x, y) != (self.x2, self.y2) {
            ret.push((x, y));
            x += x_mod;
            y += y_mod;
        }
        ret.push((x, y));
        ret
    }
}

impl FromStr for Segment {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (xy1, xy2) = s.split_once(" -> ").ok_or("no ->")?;
        let (x1, y1) = xy1.split_once(",").ok_or("no comma")?;
        let (x2, y2) = xy2.split_once(",").ok_or("no comma")?;
        let x1 = x1.parse()?;
        let y1 = y1.parse()?;
        let x2 = x2.parse()?;
        let y2 = y2.parse()?;
        Ok(Segment { x1, y1, x2, y2 })
    }
}

fn count_danger_zones(segments: &Vec<Segment>, directions: &[Direction]) -> usize {
    let positions = segments
        .iter()
        .filter(|s| directions.contains(&s.direction()))
        .flat_map(|s| s.positions());
    let mut counter: HashMap<(i32, i32), i32> = HashMap::new();
    for p in positions {
        *counter.entry(p).or_insert(0) += 1;
    }
    counter.iter().filter(|(_, v)| **v >= 2).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let segments: Vec<Segment> = file_content
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_, _>>()?;

    println!("part1 result: {}", count_danger_zones(&segments, &[Direction::Horizontal, Direction::Vertical]));
    println!("part2 result: {}", count_danger_zones(&segments, &[Direction::Horizontal, Direction::Vertical, Direction::Diagonal]));
    Ok(())
}
