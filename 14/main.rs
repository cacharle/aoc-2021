use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "14/input.txt";

#[derive(Debug)]
struct PolymerRule {
    pair: (char, char),
    inserted: char,
}

impl FromStr for PolymerRule {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pair_str, inserted_str) = s.split_once(" -> ").unwrap();
        let pair = (
            pair_str.chars().nth(0).unwrap(),
            pair_str.chars().nth(1).unwrap(),
        );
        let inserted = inserted_str.chars().nth(0).unwrap();
        Ok(PolymerRule { pair, inserted })
    }
}

#[derive(Debug)]
struct PolymerPart1(Vec<char>);

impl FromStr for PolymerPart1 {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PolymerPart1(s.chars().collect()))
    }
}

#[derive(Debug, Clone)]
struct PolymerPart2 {
    counter: HashMap<(char, char), usize>,
    first_pair: (char, char),
    last_pair: (char, char),
}

impl FromStr for PolymerPart2 {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counter = HashMap::new();
        for c1 in 'A'..='Z' {
            for c2 in 'A'..='Z' {
                counter.insert((c1, c2), 0);
            }
        }
        let pairs = s.chars().zip(s.chars().skip(1));
        for pair in pairs.clone() {
            counter.insert(pair, counter[&pair] + 1);
        }
        Ok(PolymerPart2 {
            counter,
            first_pair: pairs.clone().next().unwrap(),
            last_pair: pairs.clone().last().unwrap(),
        })
    }
}

impl PolymerPart1 {
    fn step(&self, rules: &Vec<PolymerRule>) -> PolymerPart1 {
        let mut polymer: Vec<char> = Vec::new();
        for (first, second) in self.0.iter().zip(self.0.iter().skip(1)) {
            // println!("{} {}", first, second);
            polymer.push(*first);
            if let Some(rule) = rules.iter().filter(|r| r.pair == (*first, *second)).next() {
                polymer.push(rule.inserted);
            }
        }
        polymer.push(self.0[self.0.len() - 1]);
        PolymerPart1(polymer)
    }
}

impl PolymerPart2 {
    fn step(&mut self, rules: &Vec<PolymerRule>) {
        let mut new = self.clone();
        for c1 in 'A'..='Z' {
            for c2 in 'A'..='Z' {
                new.counter.insert((c1, c2), 0);
            }
        }
        for (pair, count) in self.counter.iter() {
            if *count == 0 {
                continue;
            }
            for rule in rules {
                if rule.pair == *pair {
                    let left = (pair.0, rule.inserted);
                    let right = (rule.inserted, pair.1);
                    if *pair == self.first_pair {
                        new.first_pair = left;
                    }
                    if *pair == self.last_pair {
                        new.last_pair = right;
                    }
                    new.counter.insert(left, new.counter[&left] + *count);
                    new.counter.insert(right, new.counter[&right] + *count);
                }
            }
        }
        *self = new;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let (polymer_str, polymer_rules_str) = file_content.split_once("\n\n").unwrap();

    let polymer: PolymerPart1 = polymer_str.parse()?;
    let polymer_rules: Vec<PolymerRule> = polymer_rules_str
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_, _>>()?;

    let steps = 10;
    let polymer = (0..steps).fold(polymer, |p, _| p.step(&polymer_rules));
    let mut counter = HashMap::new();
    for c in polymer.0 {
        *counter.entry(c).or_insert(0) += 1;
    }
    let most_common = counter.values().max().unwrap();
    let least_common = counter.values().min().unwrap();
    println!("part1 result: {}", most_common - least_common);

    let mut polymer: PolymerPart2 = polymer_str.parse()?;
    for _ in 0..40 {
        polymer.step(&polymer_rules);
    }
    let mut counter = HashMap::new();
    for ((c1, c2), count) in polymer.counter.iter() {
        *counter.entry(c1).or_insert(0) += count;
        *counter.entry(c2).or_insert(0) += count;
    }
    counter.insert(&polymer.first_pair.0, counter[&polymer.first_pair.0] + 1);
    counter.insert(&polymer.last_pair.1, counter[&polymer.last_pair.1] + 1);
    for (k, v) in counter.clone().iter() {
        counter.insert(*k, v / 2);
    }
    let most_common = counter.values().filter(|v| **v != 0).max().unwrap();
    let least_common = counter.values().filter(|v| **v != 0).min().unwrap();
    println!("part2 result: {}", most_common - least_common);

    Ok(())
}
