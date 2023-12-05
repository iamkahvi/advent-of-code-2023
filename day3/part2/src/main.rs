use regex::Regex;
use std::cmp::min;
use std::fmt;

fn main() {
    let lines = include_str!("../../input1.txt")
        .lines()
        .collect::<Vec<&str>>();

    let grid = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            // println!("{}", line);

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

                            let regex_match = re
                                .find_iter(line_segment)
                                .max_by_key(|mat| mat.as_str().len())
                                .unwrap();

                            // println!("{}, {:?}", j, regex_match);

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
                                id: 0,
                            };

                            Cell {
                                kind: CellType::Digit(d),
                                i: i,
                                j: j,
                            }
                        }
                        Ok(CellType::Symbol(_)) => Cell {
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

    let mut total: Vec<i32> = vec![];

    for (i, row) in grid.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell.kind {
                CellType::Symbol(c) => {
                    if c == '*' {
                        println!("\nfound a gear at ({},{})", cell.i, cell.j);

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

                        let top_line_numbers =
                            get_numbers_from_line(&tl.kind, &above.kind, &tr.kind);

                        let bottom_line_numbers =
                            get_numbers_from_line(&bl.kind, &below.kind, &br.kind);

                        let middle_line_numbers = get_numbers_from_middle(&left.kind, &right.kind);

                        let nums = top_line_numbers
                            .iter()
                            .chain(bottom_line_numbers.iter())
                            .chain(middle_line_numbers.iter())
                            .collect::<Vec<&i32>>();

                        let len_nums = nums.len();
                        let nums_clone = nums.clone();

                        if len_nums == 2 as usize {
                            let product = nums_clone.iter().fold(1, |acc, num| acc * num.clone());
                            total.push(product.clone());
                        }

                        println!(
                            "{}{}{}\n{} {}\n{}{}{}",
                            tl, above, tr, left, right, bl, below, br
                        );
                        dbg!(nums);
                    }
                }
                _ => (),
            }
        }
    }

    println!("{:?}", total);
    println!("{:?}", total.iter().sum::<i32>());
}

fn get_numbers_from_middle(left: &CellType, right: &CellType) -> Vec<i32> {
    match (left, right) {
        (CellType::Digit(d1), CellType::Digit(d2)) => {
            vec![d1.number.unwrap(), d2.number.unwrap()]
        }
        (CellType::Digit(d), _) => {
            vec![d.number.unwrap()]
        }
        (_, CellType::Digit(d)) => {
            vec![d.number.unwrap()]
        }
        _ => vec![],
    }
}

fn get_numbers_from_line(left: &CellType, middle: &CellType, right: &CellType) -> Vec<i32> {
    match (left, middle, right) {
        (CellType::Digit(d1), CellType::Digit(_), CellType::Digit(_)) => {
            vec![d1.number.unwrap()]
        }
        (CellType::Digit(d1), CellType::Digit(_), _) => {
            vec![d1.number.unwrap()]
        }
        (_, CellType::Digit(d1), CellType::Digit(_)) => {
            vec![d1.number.unwrap()]
        }
        (CellType::Digit(d1), CellType::Blank, CellType::Digit(d2))
        | (CellType::Digit(d1), CellType::Symbol(_), CellType::Digit(d2)) => {
            vec![d1.number.unwrap(), d2.number.unwrap()]
        }
        (_, _, CellType::Digit(d)) => {
            vec![d.number.unwrap()]
        }
        (CellType::Digit(d), _, _) => {
            vec![d.number.unwrap()]
        }
        _ => vec![],
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
    id: usize,
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
                id: 0,
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
                CellType::Blank => ".".to_string(),
                CellType::Symbol(value) => value.to_string(),
            }
        };
        write!(f, "{}", value)
    }
}
