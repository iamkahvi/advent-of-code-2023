use regex::Regex;
use std::cmp::{max, min};
use std::fmt;

fn main() {
    let lines = include_str!("../../test1.txt")
        .lines()
        .collect::<Vec<&str>>();

    let grid = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            println!("{}", line);

            return line
                .chars()
                .enumerate()
                .map(move |(j, c)| {
                    let kind = CellType::from_char(&c);

                    match kind {
                        Ok(CellType::Digit(d)) => {
                            let re = Regex::new(r"\d+").unwrap();

                            let start = {
                                match j.checked_sub(2) {
                                    Some(y) => y,
                                    None => 0,
                                }
                            };
                            let end = min(j + 3, line.len() - 1);
                            let line_segment = &line[start..end];

                            let number = re
                                .find_iter(line_segment)
                                .max_by_key(|mat| mat.as_str().len())
                                .unwrap()
                                .as_str()
                                .parse::<i32>()
                                .ok();

                            let d = DigitCell {
                                value: d.value,
                                number,
                            };

                            Cell {
                                kind: CellType::Digit(d),
                                i: i,
                                j: j,
                            }
                        }
                        Ok(CellType::Symbol(s)) => Cell {
                            kind: kind.unwrap(),
                            i: i,
                            j: j,
                        },
                        Ok(CellType::Blank) => Cell {
                            kind: kind.unwrap(),
                            i: i,
                            j: j,
                        },
                        Err(_) => panic!("An error occurred"),
                    }
                })
                .collect::<Vec<Cell>>();
        })
        .collect::<Vec<CellRow>>();

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell.kind {
                CellType::Symbol(c) => {
                    if c == '*' {
                        println!("found a gear at {}, {}", cell.i, cell.j);

                        let row_above = &grid[{
                            match i.checked_sub(1) {
                                Some(y) => y,
                                None => 0,
                            }
                        }];
                        let row_below = &grid[min(i + 1, grid.len() - 1)];

                        let left_index = {
                            match j.checked_sub(1) {
                                Some(y) => y,
                                None => 0,
                            }
                        };
                        let right_index = min(j + 1, row.len() - 1);

                        let above = &row_above[j];
                        let below = &row_below[j];
                        let left = &row[left_index];
                        let right = &row[right_index];
                        let tl = &row_above[left_index];
                        let tr = &row_above[right_index];
                        let bl = &row_below[left_index];
                        let br = &row_below[right_index];

                        println!(
                            "{}{}{}\n{} {}\n{}{}{}",
                            tl, above, tr, left, right, bl, below, br
                        );
                    }
                }
                _ => (),
            }
        }
    }
}

type CellRow = Vec<Cell>;

// impl fmt::Display for CellRow {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.join(""))
//     }
// }

#[derive(Debug)]
struct DigitCell {
    value: usize,
    number: Option<i32>,
}

#[derive(Debug)]
enum CellType {
    Digit(DigitCell),
    Symbol(char),
    Blank,
}

impl CellType {
    fn from_char(c: &char) -> Result<Self, ()> {
        let digit_regex = Regex::new(r"^\d$").unwrap();
        let blank_regex = Regex::new(r"^\.$").unwrap();

        let s = c.to_string();

        if digit_regex.is_match(&s) {
            let d = DigitCell {
                value: s.parse::<usize>().unwrap(),
                number: None,
            };
            Ok(CellType::Digit(d))
        } else if blank_regex.is_match(&s) {
            Ok(CellType::Blank)
        } else {
            Ok(CellType::Symbol(*c))
        }
    }
}

#[derive(Debug)]
struct Cell {
    kind: CellType,
    i: usize,
    j: usize,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = {
            match &self.kind {
                CellType::Digit(d) => d.value.to_string(),
                CellType::Blank => "_".to_string(),
                CellType::Symbol(value) => value.to_string(),
            }
        };
        write!(f, "{}", value)
    }
}
