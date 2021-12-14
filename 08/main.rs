use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "08/input.txt";

#[derive(Debug)]
struct Signal(Vec<char>);
#[derive(Debug)]
struct Signals(Vec<Signal>);

#[derive(Debug)]
struct Entry {
    patterns: Signals,
    outputs: Signals,
}

impl FromStr for Signal {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signal(s.chars().collect()))
    }
}

impl FromStr for Signals {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Signals(
            s.split(" ").map(|s| s.parse()).collect::<Result<_, _>>()?,
        ))
    }
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (patterns, outputs) = s.split_once(" | ").ok_or("no | character")?;
        Ok(Entry {
            patterns: patterns.parse()?,
            outputs: outputs.parse()?,
        })
    }
}

impl Signal {
    fn possible_numbers(&self) -> Vec<i32> {
        match self.0.len() {
            2 => vec![1],
            4 => vec![4],
            3 => vec![7],
            5 => vec![2, 3, 5],
            6 => vec![0, 6, 9],
            7 => vec![8],
            _ => panic!("unknown len"),
        }
    }

    fn unique_number(&self) -> Option<i32> {
        let possible = self.possible_numbers();
        match possible.len() {
            1 => Some(possible[0]),
            _ => None,
        }
    }

    fn is_valid_perm(&self, perm: &[char]) -> bool {
        self.decode(perm).is_some()
    }

    fn decode(&self, perm: &[char]) -> Option<i32> {
        let mut positions: Vec<usize> = self
            .0
            .iter()
            .map(|c| perm.iter().position(|pc| pc == c).unwrap())
            .collect();
        positions.sort();
        match &positions[..] {
            [0, 1, 2, 4, 5, 6] => Some(0),    // 0
            [2, 5] => Some(1),                // 1
            [0, 2, 3, 4, 6] => Some(2),       // 2
            [0, 2, 3, 5, 6] => Some(3),       // 2
            [1, 2, 3, 5] => Some(4),          // 4
            [0, 1, 3, 5, 6] => Some(5),       // 5
            [0, 1, 3, 4, 5, 6] => Some(6),    // 6
            [0, 2, 5] => Some(7),             // 7
            [0, 1, 2, 3, 4, 5, 6] => Some(8), // 8
            [0, 1, 2, 3, 5, 6] => Some(9),    // 9
            _ => None,
        }
    }
}

impl Signals {
    fn is_valid_perm(&self, perm: &[char]) -> bool {
        self.0.iter().all(|s| s.is_valid_perm(perm))
    }

    fn decode(&self, perm: &[char]) -> Vec<i32> {
        self.0.iter().map(|s| s.decode(perm).unwrap()).collect()
    }

    fn decode_num(&self, perm: &[char]) -> i32 {
        self.decode(perm).iter().fold(0, |acc, x| acc * 10 + x)
    }
}

fn permutations(xs: &[char]) -> Vec<Vec<char>> {
    if xs.len() == 0 {
        return vec![];
    }
    if xs.len() == 1 {
        return vec![vec![xs[0]]];
    }
    let mut results: Vec<Vec<char>> = Vec::new();
    // println!("{:?}", xs);
    for i in 0..xs.len() {
        let elem = xs[i];
        let before = &xs[..i];
        let after = &xs[i + 1..];
        let mut rec_results = permutations(&[before, after].concat());
        // println!("{:?}", rec_results);
        for r in &mut rec_results {
            r.push(elem);
        }
        results.extend(rec_results);
    }
    results
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let entries: Vec<Entry> = file_content
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_, _>>()?;

    let unique_count = entries
        .iter()
        .flat_map(|e| {
            e.outputs.0.iter().map(|s| s.unique_number()) //o.0.iter().map(|s| s.unique_number()))
        })
        .filter(|n| n.is_some())
        .count();
    println!("part1 result: {}", unique_count);

    let segment_permutations = permutations(&"abcdefg".chars().collect::<Vec<char>>());

    let mut s = 0;
    for entry in &entries {
        for perm in &segment_permutations {
            if entry.patterns.is_valid_perm(perm) {
                let decoded_num = entry.outputs.decode_num(perm);
                s += decoded_num;
                break;
            }
        }
    }
    println!("part2 result: {}", s);

    Ok(())
}
