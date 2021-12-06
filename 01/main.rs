use std::error::Error;
use std::fs;

const INPUT_FILENAME: &str = "01/input.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let nums = file_content
        .lines()
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    let result = nums
        .iter()
        .zip(nums.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count();
    println!("part1 result: {}", result);

    let result = nums
        .iter()
        .zip(nums.iter().skip(1))
        .zip(nums.iter().skip(2))
        .zip(nums.iter().skip(3))
        .filter(|(((&a, &b), &c), &d)| (a + b + c) < (b + c + d))
        .count();
    println!("part2 result: {}", result);

    Ok(())
}
