use std::fs;
use std::error::Error;
use std::str::FromStr;

const INPUT_FILENAME: &str = "02/input.txt";

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, n) = s.split_once(' ').ok_or("bad command format")?;
        let n = n.parse::<i32>()?;
        use Command::*;
        match c {
            "forward" => Ok(Forward(n)),
            "down" => Ok(Down(n)),
            "up" => Ok(Up(n)),
            _ => Err("bad command name")?
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let commands: Vec<Command> = file_content.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;

    use Command::*;
    let horizontal_pos: i32 = commands
        .iter()
        .map(|c| match c {
             Forward(n) => *n,
             _ => 0
        })
        .sum();

    let depth: i32 = commands
        .iter()
        .map(|c| match c {
            Down(n) => *n,
            Up(n) => -n,
            _ => 0
        })
        .sum();
    println!("part1 result: {} * {} = {}", horizontal_pos, depth, horizontal_pos * depth);

    // no fancy iterator :'(
    let mut aim = 0;
    let mut depth = 0;
    for c in commands {
        match c {
            Down(n) => aim += n,
            Up(n) => aim -= n,
            Forward(n) => depth += aim * n,
        }
    }
    println!("part2 result: {} * {} = {}", horizontal_pos, depth, horizontal_pos * depth);

    Ok(())
}
