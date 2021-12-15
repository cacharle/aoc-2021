use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

const INPUT_FILENAME: &str = "10/input.txt";

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum DelimiterKind {
    Parent,
    Square,
    Curly,
    Arrow,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Delimiter {
    open: bool,
    kind: DelimiterKind,
}

impl Delimiter {
    fn from_char(c: char) -> Delimiter {
        use DelimiterKind::*;
        match c {
            '(' => Delimiter {
                open: true,
                kind: Parent,
            },
            '[' => Delimiter {
                open: true,
                kind: Square,
            },
            '{' => Delimiter {
                open: true,
                kind: Curly,
            },
            '<' => Delimiter {
                open: true,
                kind: Arrow,
            },
            ')' => Delimiter {
                open: false,
                kind: Parent,
            },
            ']' => Delimiter {
                open: false,
                kind: Square,
            },
            '}' => Delimiter {
                open: false,
                kind: Curly,
            },
            '>' => Delimiter {
                open: false,
                kind: Arrow,
            },
            _ => panic!("not a valid delimiter {}", c),
        }
    }

    fn pair(&self) -> Delimiter {
        Delimiter {
            open: !self.open,
            kind: self.kind,
        }
    }

    fn score_multiplier(&self) -> i64 {
        use DelimiterKind::*;
        match self.kind {
            Parent => 3,
            Square => 57,
            Curly => 1197,
            Arrow => 25137,
        }
    }

    fn autocomplete_score_adder(&self) -> i64 {
        use DelimiterKind::*;
        match self.kind {
            Parent => 1,
            Square => 2,
            Curly => 3,
            Arrow => 4,
        }
    }
}

struct Delimiters(Vec<Delimiter>);

impl FromStr for Delimiters {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Delimiters(
            s.chars().map(|c| Delimiter::from_char(c)).collect(),
        ))
    }
}

enum DelimitersResult {
    Corrupted(Delimiter),
    Incomplete(Vec<Delimiter>),
    Valid,
}

impl Delimiters {
    fn parse(&self) -> DelimitersResult {
        let mut stack = Vec::new();
        for delimiter in &self.0 {
            if delimiter.open {
                stack.push(delimiter.clone());
            } else {
                let top = *stack.last().unwrap();
                if top == delimiter.pair() {
                    stack.pop();
                } else {
                    return DelimitersResult::Corrupted(*delimiter);
                }
            }
        }
        if stack.len() != 0 {
            return DelimitersResult::Incomplete(stack);
        }
        return DelimitersResult::Valid;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(INPUT_FILENAME)?;
    let lines: Vec<Delimiters> = file_content
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_, _>>()?;

    let mut counter = HashMap::new();
    let mut autocomplete_scores = Vec::new();
    for delimiters in &lines {
        match delimiters.parse() {
            DelimitersResult::Corrupted(delimiter) => *counter.entry(delimiter).or_insert(0) += 1,
            DelimitersResult::Incomplete(not_closed) => {
                let mut score = 0;
                for d in not_closed.iter().rev() {
                    score *= 5;
                    score += d.autocomplete_score_adder()
                }
                autocomplete_scores.push(score);
            }
            DelimitersResult::Valid => panic!("should not be valid"),
        }
    }

    let mut sum = 0;
    for (delimiter, count) in counter {
        sum += count * delimiter.score_multiplier();
    }
    println!("part1 result: {}", sum);

    autocomplete_scores.sort();
    println!(
        "part2 result: {}",
        autocomplete_scores[autocomplete_scores.len() / 2]
    );

    Ok(())
}
