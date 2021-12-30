use std::fs;
use std::error::Error;
use std::ops::Range;
use std::str::FromStr;

const INPUT_FILENAME: &str = "17/input-example.txt";

struct TargetArea {
    x_range: Range<i64>,
    y_range: Range<i64>,
}

impl FromStr for TargetArea {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_range_str, y_range_str) = s
            .trim()
            .strip_prefix("target area: ")
            .ok_or("no prefix")?
            .split_once(", ")
            .ok_or("no ', '")?;
        let x_range_str = x_range_str.strip_prefix("x=").ok_or("no x= prefix")?;
        let y_range_str = y_range_str.strip_prefix("y=").ok_or("no y= prefix")?;
        let (x_start, x_end) = x_range_str.split_once("..").ok_or("no ..")?;
        let (y_start, y_end) = y_range_str.split_once("..").ok_or("no ..")?;
        Ok(TargetArea {
            x_range: x_start.parse()?..x_end.parse()?,
            y_range: y_start.parse()?..y_end.parse()?,
        })
    }
}

impl TargetArea {
    fn contains(&self, probe: &Probe) -> bool {
        self.x_range.contains(&probe.x) && self.y_range.contains(&probe.y)
    }
}

struct Probe {
    x: i64,
    y: i64,
    x_velocity: i64,
    y_velocity: i64,
    y_max: i64,
}

impl Probe {
    fn new(x_velocity: i64, y_velocity: i64) -> Probe {
        Probe {
            x: 0,
            y: 0,
            y_max: 0,
            x_velocity,
            y_velocity
        }
    }

    fn step(&mut self) {
        self.x += self.x_velocity;
        self.y += self.y_velocity;
        self.x_velocity += match self.x_velocity.cmp(&0) {
            Less => 1,
            Greater => -1,
            Equal => 0,
        };
        self.y_velocity -= 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let target_area: TargetArea = file_content.parse()?;

    println!("x={:?} y={:?}", target_area.x_range, target_area.y_range);
    println!("x={:?} y={:?}", target_area.x_range.end, target_area.y_range.end);

    for x in 0..target_area.x_range.end {
        for y in 0..target_area.y_range.end {
            println!("{} {}", x, y);
        }
    }

    Ok(())
}
