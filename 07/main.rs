use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "07/input.txt";

#[derive(Clone)]
struct Crabs(Vec<i32>);

impl Crabs {
    fn fuel_cost_part1(&self, pos: i32) -> i32 {
        self.0.iter().map(|p| (p - pos).abs()).sum()
    }

    /*
     * PERF: can probably replace the summation in map by a smarter formula
     *       right now, we need -O3, it taks 45s with -O0
     * f(x) = f(x - 1) + 1
     * f(1) = 1
     * Indeed: it's just the triangular numbers (thx Maxime)
     */
    fn fuel_cost_part2(&self, pos: i32) -> i32 {
        self.0
            .iter()
            // previous solution in O(n^2): .map(|p| (1..(p - pos).abs() + 1).sum::<i32>())
            .map(|p| Crabs::triangular((p - pos).abs()))
            .sum()
    }

    fn triangular(n: i32) -> i32 {
        let n = n + 1;
        (n * (n - 1)) / 2
    }
}

impl FromStr for Crabs {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .trim()
            .split(",")
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        Ok(Crabs(nums))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let crabs: Crabs = file_content.parse()?;

    let min = *crabs.0.iter().min().ok_or("no crabs pos")?;
    let max = *crabs.0.iter().max().ok_or("no crabs pos")?;

    let min_fuel_cost = (min..max)
        .map(|pos| crabs.fuel_cost_part1(pos))
        .min()
        .ok_or("couldn't find min fuel cost")?;
    println!("part1 result: {}", min_fuel_cost);

    let min_fuel_cost = (min..max)
        .map(|pos| crabs.fuel_cost_part2(pos))
        .min()
        .ok_or("couldn't find min fuel cost")?;
    println!("part2 result: {}", min_fuel_cost);

    Ok(())
}
