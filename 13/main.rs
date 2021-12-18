use std::error::Error;
use std::fmt;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "13/input.txt";

#[derive(Clone)]
struct Sheet(Vec<Vec<bool>>);

impl fmt::Debug for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &x in row {
                write!(f, "{}", if x { "#" } else { "." })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl FromStr for Sheet {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dots: Vec<(usize, usize)> = s
            .lines()
            .map(|l| l.split_once(",").unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
            .collect();
        let width = dots.iter().map(|t| t.0).max().unwrap() + 1;
        let height = dots.iter().map(|t| t.1).max().unwrap() + 1;
        let mut sheet = vec![vec![false; width]; height];
        for d in dots {
            sheet[d.1][d.0] = true;
        }
        Ok(Sheet(sheet))
    }
}

#[derive(Clone, Debug)]
struct FoldInstruction {
    axis: char,
    index: usize,
}

impl FromStr for FoldInstruction {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, index) = s
            .strip_prefix("fold along ")
            .unwrap()
            .split_once("=")
            .unwrap();
        Ok(FoldInstruction {
            axis: axis.chars().next().unwrap(),
            index: index.parse().unwrap(),
        })
    }
}

impl Sheet {
    fn transpose(&self) -> Sheet {
        let mut sheet = Sheet(vec![vec![false; self.0.len()]; self.0[0].len()]);
        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                sheet.0[j][i] = self.0[i][j];
            }
        }
        sheet
    }

    fn fold(&self, instruction: &FoldInstruction) -> Sheet {
        let folded_sheet = if instruction.axis == 'x' {
            self.transpose()
        } else {
            self.clone()
        };

        let mut fold_dest = Sheet(folded_sheet.0[..instruction.index].to_vec());
        let fold_src = Sheet(folded_sheet.0[instruction.index + 1..].to_vec());
        for dest_pos in 0..fold_dest.0.len() {
            let src_pos = fold_dest.0.len() - dest_pos - 1;
            for i in 0..fold_dest.0[dest_pos].len() {
                if fold_src.0[src_pos][i] {
                    fold_dest.0[dest_pos][i] = fold_src.0[src_pos][i];
                }
            }
        }
        if instruction.axis == 'x' {
            fold_dest.transpose()
        } else {
            fold_dest
        }
    }

    fn count_visible_dots(&self) -> usize {
        self.0.iter().flatten().filter(|&&d| d).count()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;

    let (sheet_str, fold_instructions_str) = file_content.split_once("\n\n").unwrap();
    let sheet: Sheet = sheet_str.parse()?;
    let fold_instructions: Vec<FoldInstruction> = fold_instructions_str
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_, _>>()?;

    println!(
        "part1 result: {}",
        sheet.fold(&fold_instructions[0]).count_visible_dots()
    );

    let folded = fold_instructions
        .iter()
        .fold(sheet, |sheet, instruction| sheet.fold(instruction));
    println!("part2 result:");
    println!("{:?}", folded);

    Ok(())
}
