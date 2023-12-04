use regex::Regex;

fn main() {
    let lines = include_str!("../../test1.txt")
        .lines()
        .collect::<Vec<&str>>();

    let grid = lines.iter().map(|line| {
        println!("{}", line);

        return line.chars().map(|c| {
            let kind = CellType::from_char(&c);

            match kind {
                Ok(CellType::Digit(d)) => {
                    println!("Success! It's a digit: {}", d)
                }
                Ok(CellType::Symbol(s)) => {
                    println!("Success! It's a symbol: {}", s)
                }
                Ok(CellType::Blank) => {
                    println!("Success! It's a blank")
                }
                Err(_) => panic!("An error occurred"),
            }
        });
        // dbg!(solve_line(line).unwrap());
    });
}

#[derive(Debug)]
enum CellType {
    Digit(i32),
    Symbol(char),
    Blank,
}

impl CellType {
    fn from_char(c: &char) -> Result<Self, ()> {
        let digit_regex = Regex::new(r"^\d$").unwrap();
        let blank_regex = Regex::new(r"^\.$").unwrap();

        let s = c.to_string();

        if digit_regex.is_match(&s) {
            Ok(CellType::Digit(s.parse::<i32>().unwrap()))
        } else if blank_regex.is_match(&s) {
            Ok(CellType::Blank)
        } else {
            Ok(CellType::Symbol(*c))
        }
    }
}

struct Cell {
    kind: CellType,
    i: usize,
    j: usize,
    grid: Vec<Vec<char>>,
}

impl Cell {}
