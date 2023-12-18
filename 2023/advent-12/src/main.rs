use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    // . = operational
    // ? = unknown
    // # = damaged
    // calc the number of permutations that satisfy the result
    // ???.### 1,1,3      = 1
    // ?###???????? 3,2,1 = 10
    // add all rows no. permutations for day one = 21

    let mut lines = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.push(line);
                
            }
            _ => {}
        }
    }

    dayOne = lines.par_iter().map(|line| get_matches(line.clone())).sum();

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

fn get_matches(line: String) -> usize {
    let mut res = 0;

    let parts = line.split(" ").collect::<Vec<&str>>();
    let matches = parts[1].split(",").map(|str| str.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let row = parts[0].chars().map(|str| str.to_string()).collect::<Vec<String>>();

    // println!("Start : {}", row.join(""));

    let options = replace(row);

    // println!("options {} : {:?}", options.len(), options);

    for option in options {
        let check = option
            .split(".")
            .map(|str| str.replace(".", "").trim().len()).filter(|len| len > &0).collect::<Vec<usize>>();

        if check.len() == matches.len() {
            if (0..check.len()).into_iter().all(|i| check[i] == matches[i]) {
                res += 1;
            }
        }
    }

    return res;
}

fn replace(line: Vec<String>) -> HashSet<String> {
    let mut collector = HashSet::new();
    replace_at(line, 0, &mut collector);

    // println!("collector -> {:?}", collector);

    return collector;
}

fn replace_at(line: Vec<String>, pos: usize, collector: &mut HashSet<String>) {
    let chars: Vec<String> = vec!["#", "."].iter().map(|str| str.to_string()).collect();

    for char in chars.iter() {
        let mut newline = line.clone();
        if line[pos] == "?" {
            newline[pos] = char.clone();
        }

        if pos + 1 == line.len() {
            collector.insert(newline.join(""));
        } else {
            replace_at(newline, pos + 1, collector);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{get_matches, replace};

    #[test]
    pub fn test_replace() {
        assert_eq!(replace("???.###".chars().map(|str| str.to_string()).collect()).len(), 666);
    }

    #[test]
    pub fn test_row_1() {
        assert_eq!(get_matches("???.### 1,1,3".into()), 1);
    }

    #[test]
    pub fn test_row_2() {
        assert_eq!(get_matches(".??..??...?##. 1,1,3".into()), 4);
    }

    #[test]
    pub fn test_row_6() {
        assert_eq!(get_matches("?###???????? 3,2,1".into()), 10);
    }

    #[test]
    pub fn test_check_match() {
        let matches: Vec<usize> = vec![1, 1, 3];
        let option = ".#....#...###.";
        let check = option
            .split(".")
            .map(|str| str.replace(".", "").trim().len()).filter(|len| len > &0).collect::<Vec<usize>>();

        println!("Check {} -> {:?}", check.len(), check);

        if check.len() == matches.len() {
            if (0..check.len()).into_iter().all(|i| check[i] == matches[i]) {
                println!("SUCCESS");
            }
        }
    }
}
