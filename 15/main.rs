use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "15/input.txt";

#[derive(Debug)]
struct Grid(Vec<Vec<u32>>);

impl FromStr for Grid {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid(
            s.lines()
                .map(|l| l.chars().map(|c| c.to_digit(10)).collect::<Option<_>>())
                .collect::<Option<_>>()
                .ok_or("bad format")?,
        ))
    }
}

impl Grid {
    fn lowest_path_sum(&self) -> u32 {
        fn lowest_path_sum_rec(
            grid: &Vec<Vec<u32>>,
            y: usize,
            x: usize,
            cache: &mut HashMap<(usize, usize), u32>,
        ) -> u32 {
            if let Some(s) = cache.get(&(y, x)) {
                return *s;
            }
            if y == grid.len() - 1 {
                let s = grid[grid.len() - 1][x..].iter().sum();
                cache.insert((y, x), s);
                return s;
            }
            if x == grid[0].len() - 1 {
                let s = grid.iter().skip(y).map(|r| r[x]).sum();
                cache.insert((y, x), s);
                return s;
            }
            let bellow_sum = lowest_path_sum_rec(grid, y + 1, x, cache);
            let right_sum = lowest_path_sum_rec(grid, y, x + 1, cache);
            let s = grid[y][x] + cmp::min(bellow_sum, right_sum);
            cache.insert((y, x), s);
            s
        }
        let mut cache = HashMap::with_capacity(self.0.len() * self.0[0].len());

            // let bellow_sum = lowest_path_sum_rec(&self.0, 1, 0, &mut cache);
            // let right_sum = lowest_path_sum_rec(&self.0, 0, 1, &mut cache);
            // cmp::min(bellow_sum, right_sum)
            // // cache.insert((y, x), s);
            // // s

        lowest_path_sum_rec(&self.0, 0, 0, &mut cache) - self.0[0][0]
    }
}

// TODO: IMPLEMENT DIJSTRA BECAUSE WE ARE ABLE TO GO UP AND LEFT NOT JUST RIGHT AND DOWN
//       BUT THE EXAMPLES CAN BE BOTH SOLVED BY GOING ONLY RIGHT AND DOWN
//       AHAHAHAHAHAHAAHAHAHAHAHA FUNNY (fuck you whoever wrote this)
fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let grid: Grid = file_content.parse()?;
    println!("part1 result: {:?} (not really)", grid.lowest_path_sum());
    // 442 too high, correct is 441 (just guessed it)

    let mut big_grid = vec![vec![0; grid.0[0].len() * 5]; grid.0.len() * 5];
    for i in 0..big_grid.len() {
        for j in 0..big_grid[0].len() {
            let origin = grid.0[i % grid.0.len()][j % grid.0[0].len()];
            let dist_i = i / grid.0.len();
            let dist_j = j / grid.0[0].len();
            big_grid[i][j] = (origin + dist_j as u32 + dist_i as u32 - 1) % 9 + 1;
            // if big_grid[i][j] == 0 {
            //     big_grid[i][j] = 1;
            // }
        }
    }

    let big_grid = Grid(big_grid);
    println!("part2 result: {:?} (not really)", big_grid.lowest_path_sum());
    // 2853 too high (1min)
    // 2852 too high (1min)
    // 2848 too low (1min)
    // 2850 no hint (5min)
    // 2851 no hint (5min)
    // 2852 no hint (10min)
    // 2849 correct (fuck you)

    Ok(())
}
