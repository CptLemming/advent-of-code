use std::{fs, path::Path};
use regex::Regex;

type Row = String;
type Rows = Vec<Row>;

#[derive(Debug)]
enum Statement {
    Multiply(Multiply),
    Do,
    DoNot,
}

#[derive(Debug)]
struct Multiply {
    left: u32,
    right: u32,
}

impl Multiply {
    pub fn get_value(&self) -> u64 {
        self.left as u64 * self.right as u64
    }
}
type Program = Vec<Statement>;

fn main() {
    let answer = step_one("input.txt");
    println!("Step one is : {answer}");
    let answer = step_two("input.txt");
    println!("Step two is : {answer}");
}

fn step_one(file_name: &str) -> u64 {
    let instructions = load_file(file_name);
    let mut is_enabled = true;
 
    instructions.into_iter().map(|row| parse_row(row)).map(|row| sum_entries(row, false, &mut is_enabled)).sum()
}
 
fn step_two(file_name: &str) -> u64 {
    let instructions = load_file(file_name);
    let mut is_enabled = true;
 
    instructions.into_iter().map(|row| parse_row(row)).map(|row| sum_entries(row, true, &mut is_enabled)).sum()
}

fn load_file(file_name: &str) -> Rows {
    let p = Path::new(file_name);

    let content = fs::read_to_string(p).expect("Failed to read file");

    content
        .lines()
        .into_iter()
        .map(|line| line.to_string())
        .collect()
}

fn parse_row(row: Row) -> Program {
    let mut out = vec![];

    let regex = Regex::new(r"(?m)((?P<mul>mul)\((?P<left>\d+),(?P<right>\d+)\))|(?P<do>do\(\))|(?P<donot>don't\(\))").unwrap();

    for entry in regex.captures_iter(&row) {
        if let Some(_) = entry.name("mul") {
            match (entry["left"].parse::<u32>(), entry["right"].parse::<u32>()) {
                (Ok(left), Ok(right)) => {
                    out.push(Statement::Multiply(Multiply{ left, right }));
                }
                _ => {}
            }
        }
        if let Some(_) = entry.name("do") {
            out.push(Statement::Do);
        }
        if let Some(_) = entry.name("donot") {
            out.push(Statement::DoNot);
        }

    }

    out
}

fn sum_entries(entries: Program, do_donot: bool, is_enabled: &mut bool) -> u64 {
    let mut out = 0;

    for entry in entries {
        match entry {
            Statement::Multiply(mul) => {
                if *is_enabled {
                    out += mul.get_value();
                }
            }
            Statement::Do => {
                *is_enabled = true;
            }
            Statement::DoNot => {
                if do_donot {
                    *is_enabled = false;
                }
            }
        }
    }

    out
}

#[cfg(test)]
mod test {
    use crate::{parse_row, step_one, step_two};

    #[test]
    pub fn test_parse_row() {
        let entires = parse_row("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)".to_owned());
        println!("Entries : {:?}", entires);
    }

    #[test]
    pub fn test_step_one() {
        let answer = step_one("test.txt");
        assert_eq!(answer, 161);
    }

    #[test]
    pub fn test_step_two() {
        let answer = step_two("test2.txt");
        assert_eq!(answer, 48);
    }
}
