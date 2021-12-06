use std::error::Error;
use std::fs;

const INPUT_FILENAME: &str = "03/input.txt";

fn bit_count(bits: &Vec<u32>) -> (usize, usize) {
    let one_count = bits.iter().filter(|b| **b == 1).count();
    (one_count, bits.len() - one_count)
}

fn most_common_bit(bits: &Vec<u32>) -> u32 {
    let (one_count, zero_count) = bit_count(bits);
    if one_count > zero_count {
        1
    } else {
        0
    }
}

fn part2<T>(nums: &Vec<Vec<u32>>, f: T) -> u32
where
    T: Fn(usize, usize) -> u32,
{
    let mut nums_copy: Vec<Vec<u32>> = nums.iter().cloned().collect();
    let mut i = 0;
    while nums_copy.len() > 1 {
        let bits = nums_copy.iter().map(|n| n[i]).collect();
        let (one_count, zero_count) = bit_count(&bits);
        let target = f(one_count, zero_count);
        nums_copy = nums_copy
            .iter()
            .filter(|bs| bs[i] == target)
            .cloned()
            .collect();
        i += 1;
    }
    nums_copy[0].iter().fold(0, |acc, b| (acc << 1) | b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;

    let nums: Vec<Vec<u32>> = file_content
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10)).collect())
        .collect::<Option<_>>()
        .ok_or("cannot parse bit")?;

    // https://stackoverflow.com/questions/64498617
    let nums_transpose: Vec<Vec<u32>> = (0..nums[0].len())
        .map(|i| nums.iter().map(|inner| inner[i].clone()).collect())
        .collect();

    let mut gamma = 0;
    let mut epsilon = 0;
    for bits in nums_transpose.iter() {
        let most_common = most_common_bit(&bits);
        gamma = (gamma << 1) | most_common;
        epsilon = (epsilon << 1) | ((!most_common) & 1);
    }
    println!("part1 result: {}", gamma * epsilon);

    // terrible code, but I learned how to pass closure as arguments so yaay
    let oxygen_generator_rating = part2(
        &nums,
        |one_count, zero_count| if one_count >= zero_count { 1 } else { 0 },
    );
    let c02_scrubber_rating = part2(
        &nums,
        |one_count, zero_count| if zero_count <= one_count { 0 } else { 1 },
    );

    println!(
        "part2 result: {}",
        c02_scrubber_rating * oxygen_generator_rating
    );

    Ok(())
}
