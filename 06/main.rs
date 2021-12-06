use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "06/input.txt";

///////////////////////////////////////////////////////////////////////////////
// part1 solution                                                            //
///////////////////////////////////////////////////////////////////////////////
struct LanternFishPool(Vec<usize>);

impl LanternFishPool {
    fn next_day(&mut self) {
        let mut new_fishes = Vec::new();
        for i in 0..self.0.len() {
            if self.0[i] == 0 {
                new_fishes.push(8);
                self.0[i] = 6;
            } else {
                self.0[i] -= 1;
            }
        }
        self.0.extend(new_fishes);
    }
}

impl FromStr for LanternFishPool {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .trim()
            .split(",")
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        Ok(LanternFishPool(nums))
    }
}

///////////////////////////////////////////////////////////////////////////////
// part2 solution                                                            //
///////////////////////////////////////////////////////////////////////////////
struct LanternFishPoolDays(Vec<usize>);

impl LanternFishPoolDays {
    fn next_day(&mut self) {
        let born = self.0[0];
        for i in 1..self.0.len() {
            self.0[i - 1] = self.0[i];
        }
        self.0[6] += born;
        self.0[8] = born;
    }
}

impl FromStr for LanternFishPoolDays {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s
            .trim()
            .split(",")
            .map(|n| n.parse())
            .collect::<Result<_, _>>()?;
        let mut lookup = vec![0; 10];
        for n in nums {
            lookup[n as usize] += 1;
        }
        Ok(LanternFishPoolDays(lookup))
    }
}
///////////////////////////////////////////////////////////////////////////////

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;

    let mut fishes: LanternFishPool = file_content.parse()?;
    for _ in 0..80 {
        fishes.next_day();
    }
    println!("part1 result: {}", fishes.0.len());

    let mut fishes_days: LanternFishPoolDays = file_content.parse()?;
    for _ in 0..256 {
        fishes_days.next_day();
    }
    println!("part2 result: {}", fishes_days.0.iter().sum::<usize>());

    Ok(())
}
