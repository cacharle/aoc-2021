use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "04/input.txt";

const BOARD_SIZE: usize = 5;

#[derive(Clone)]
struct Board {
    nums: Vec<Vec<u32>>,
    marks: Vec<Vec<bool>>,
}

impl Board {
    fn bingo(&self) -> bool {
        let transpose: Vec<Vec<bool>> = (0..BOARD_SIZE)
            .map(|i| self.marks.iter().map(|l| l[i]).collect())
            .collect();
        self.marks.iter().any(|l| l.iter().all(|m| *m))
            || transpose.iter().any(|l| l.iter().all(|m| *m))
    }

    fn mark(&mut self, num: u32) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.nums[i][j] == num {
                    self.marks[i][j] = true;
                }
            }
        }
    }

    fn score(&self, num: u32) -> u32 {
        let sum: u32 = self
            .nums
            .iter()
            .flatten()
            .zip(self.marks.iter().flatten())
            .filter(|(_, m)| !**m)
            .map(|(n, _)| n)
            .sum();
        sum * num
    }
}

impl FromStr for Board {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<Vec<u32>> = s
            .lines()
            .map(|l| {
                l.split_whitespace()
                    .map(|n| n.parse())
                    .collect::<Result<_, _>>()
            })
            .collect::<Result<_, _>>()?;
        let len = nums.len();
        Ok(Board {
            nums,
            marks: vec![vec![false; len]; len],
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let (first_line, rest) = file_content.split_once("\n").ok_or("no first line")?;
    let draws: Vec<u32> = first_line
        .split(",")
        .map(|n| n.parse())
        .collect::<Result<_, _>>()?;

    let mut boards: Vec<Board> = rest
        .trim()
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<_, _>>()?;

    // not clean, but I learned that you can put lifetimes on loops so yaay
    let mut won_indexes = vec![false; boards.len()];
    'draw_loop: for drawn in draws {
        for (i, b) in boards.iter_mut().enumerate() {
            b.mark(drawn);
            if b.bingo() {
                if won_indexes.iter().filter(|w| **w).count() == 0 {
                    println!("part1 result: {}", b.score(drawn));
                }
                won_indexes[i] = true;
                if won_indexes.iter().filter(|w| !**w).count() == 0 {
                    println!("part2 result: {}", b.score(drawn));
                    break 'draw_loop;
                }
            }
        }
    }
    Ok(())
}
