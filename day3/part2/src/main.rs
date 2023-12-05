use regex::Regex;
use std::cmp::min;
use std::fmt;

fn main() {
    let lines = include_str!("../../test2.txt")
        .lines()
        .collect::<Vec<&str>>();

    let grid = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
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
                            let end = min(j + 2, line.len() - 1);
                            let line_segment = &line[start..(end + 1)];

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

                        let row_above = {
                            match i.checked_sub(1) {
                                Some(y) => Some(&grid[y]),
                                None => None,
                            }
                        };
                        let row_below = {
                            if i + 1 <= grid.len() - 1 {
                                Some(&grid[i + 1])
                            } else {
                                None
                            }
                        };

                        let left_index = j.checked_sub(1);
                        let right_index = {
                            if j + 1 <= row.len() - 1 {
                                Some(j + 1)
                            } else {
                                None
                            }
                        };

                        let above = if let Some(ra) = row_above {
                            Some(&ra[j].kind)
                        } else {
                            None
                        };
                        let below = if let Some(rb) = row_below {
                            Some(&rb[j].kind)
                        } else {
                            None
                        };
                        let left = if let Some(li) = left_index {
                            Some(&row[li].kind)
                        } else {
                            None
                        };
                        let right = if let Some(ri) = right_index {
                            Some(&row[ri].kind)
                        } else {
                            None
                        };
                        let tl = {
                            match (row_above, left_index) {
                                (Some(ra), Some(li)) => Some(&ra[li].kind),
                                _ => None,
                            }
                        };
                        let tr = {
                            match (row_above, right_index) {
                                (Some(ra), Some(ri)) => Some(&ra[ri].kind),
                                _ => None,
                            }
                        };
                        let bl = {
                            match (row_below, left_index) {
                                (Some(rb), Some(li)) => Some(&rb[li].kind),
                                _ => None,
                            }
                        };
                        let br = {
                            match (row_below, right_index) {
                                (Some(rb), Some(ri)) => Some(&rb[ri].kind),
                                _ => None,
                            }
                        };

                        let top_line_numbers = get_numbers_from_line(tl, above, tr);
                        let bottom_line_numbers = get_numbers_from_line(bl, below, br);
                        let middle_line_numbers = get_numbers_from_middle(left, right);

                        let nums = top_line_numbers
                            .iter()
                            .chain(bottom_line_numbers.iter())
                            .chain(middle_line_numbers.iter())
                            .collect::<Vec<&i32>>();

                        if nums.len() == 2 {
                            let product = nums.iter().fold(1, |acc, num| acc * num.to_owned());
                            total.push(product.clone());
                        }

                        // println!(
                        //     "{}{}{}\n{} {}\n{}{}{}",
                        //     tl, above, tr, left, right, bl, below, br
                        // );
                        println!("{:?}", nums);
                    }
                }
                _ => (),
            }
        }
    }

    println!("{:?}", total);
    println!("{:?}", total.iter().sum::<i32>());
}

fn get_numbers_from_middle(left: Option<&CellType>, right: Option<&CellType>) -> Vec<i32> {
    match (left, right) {
        (Some(CellType::Digit(d1)), Some(CellType::Digit(d2))) => {
            vec![d1.number.unwrap(), d2.number.unwrap()]
        }
        (Some(CellType::Digit(d)), _) => {
            vec![d.number.unwrap()]
        }
        (_, Some(CellType::Digit(d))) => {
            vec![d.number.unwrap()]
        }
        _ => vec![],
    }
}

fn get_numbers_from_line(
    left: Option<&CellType>,
    middle: Option<&CellType>,
    right: Option<&CellType>,
) -> Vec<i32> {
    match (left, middle, right) {
        (Some(CellType::Digit(d1)), Some(CellType::Digit(_)), Some(CellType::Digit(_))) => {
            vec![d1.number.unwrap()]
        }
        (Some(CellType::Digit(d1)), Some(CellType::Digit(_)), _) => {
            vec![d1.number.unwrap()]
        }
        (_, Some(CellType::Digit(d1)), Some(CellType::Digit(_))) => {
            vec![d1.number.unwrap()]
        }
        (Some(CellType::Digit(d1)), _, Some(CellType::Digit(d2))) => {
            vec![d1.number.unwrap(), d2.number.unwrap()]
        }
        (_, _, Some(CellType::Digit(d))) => {
            vec![d.number.unwrap()]
        }
        (Some(CellType::Digit(d)), _, _) => {
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
                CellType::Blank => ".".to_string(),
                CellType::Symbol(value) => value.to_string(),
            }
        };
        write!(f, "{}", value)
    }
}
