use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    // let file = File::open("test2.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut dayOneDigits: Vec<String> = vec![];
                let mut dayTwoDigits: Vec<String> = vec![];

                let dayOneLine = line.clone();
                let dayTwoLine = line
                    .replace("one", "o1e")
                    .replace("two", "t2o")
                    .replace("three", "t3e")
                    .replace("four", "f4r")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "s7n")
                    .replace("eight", "e8t")
                    .replace("nine", "n9e");

                for char in dayOneLine.chars() {
                    if let Some(char) = parse_number(char.to_string().as_str()) {
                        dayOneDigits.push(char.to_string());
                    }
                }

                for char in dayTwoLine.chars() {
                    if let Some(char) = parse_number(char.to_string().as_str()) {
                        dayTwoDigits.push(char.to_string());
                    }
                }

                // println!("Digits 1 {:?}", dayOneDigits);
                // println!("Digits 2 {} = {:?}", line, dayTwoDigits);

                if dayOneDigits.len() >= 1 {
                    let add = [dayOneDigits.first().unwrap().clone(), dayOneDigits.last().unwrap().clone()].join("").parse::<u32>().unwrap();
                    dayOne += add;
                }
                if dayTwoDigits.len() >= 1 {
                    let add = [dayTwoDigits.first().unwrap().clone(), dayTwoDigits.last().unwrap().clone()].join("").parse::<u32>().unwrap();
                    dayTwo += add;

                    // println!("Digits 2 {} = {} = {:?}", line, add, dayTwoDigits);
                }
            }
            _ => {}
        }
    }

    println!("Day One : {dayOne}");
    println!("Day Two : {dayTwo}");
}

pub fn parse_number(str: &str) -> Option<&str> {
    if str.parse::<u32>().is_ok() {
        return Some(str);
    }
    None
}

pub fn parse_number_and_text(str: &str) -> Option<&str> {
    if str == "one" { return Some("1"); }
    if str == "two" { return Some("2"); }
    if str == "three" { return Some("3"); }
    if str == "four" { return Some("4"); }
    if str == "five" { return Some("5"); }
    if str == "six" { return Some("6"); }
    if str == "seven" { return Some("7"); }
    if str == "eight" { return Some("8"); }
    if str == "nine" { return Some("9"); }
    return parse_number(str);
}

#[cfg(test)]
mod tests {
    use crate::parse_number_and_text;

    #[test]
    fn parse_one() {
        assert_eq!(parse_number_and_text("one"), Some("1"));
    }
    #[test]
    fn parse_two() {
        assert_eq!(parse_number_and_text("two"), Some("2"));
    }
    #[test]
    fn parse_three() {
        assert_eq!(parse_number_and_text("three"), Some("3"));
    }
    #[test]
    fn parse_four() {
        assert_eq!(parse_number_and_text("four"), Some("4"));
    }
    #[test]
    fn parse_five() {
        assert_eq!(parse_number_and_text("five"), Some("5"));
    }
    #[test]
    fn parse_six() {
        assert_eq!(parse_number_and_text("six"), Some("6"));
    }
    #[test]
    fn parse_seven() {
        assert_eq!(parse_number_and_text("seven"), Some("7"));
    }
    #[test]
    fn parse_eight() {
        assert_eq!(parse_number_and_text("eight"), Some("8"));
    }
    #[test]
    fn parse_nine() {
        assert_eq!(parse_number_and_text("nine"), Some("9"));
    }
    #[test]
    fn parse_digit() {
        assert_eq!(parse_number_and_text("0"), Some("0"));
        assert_eq!(parse_number_and_text("1"), Some("1"));
        assert_eq!(parse_number_and_text("2"), Some("2"));
        assert_eq!(parse_number_and_text("3"), Some("3"));
        assert_eq!(parse_number_and_text("4"), Some("4"));
        assert_eq!(parse_number_and_text("5"), Some("5"));
        assert_eq!(parse_number_and_text("6"), Some("6"));
        assert_eq!(parse_number_and_text("7"), Some("7"));
        assert_eq!(parse_number_and_text("8"), Some("8"));
        assert_eq!(parse_number_and_text("9"), Some("9"));
    }
}
