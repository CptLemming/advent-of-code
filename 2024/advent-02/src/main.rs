use std::{fs, path::Path};

type Row = Vec<u32>;
type Rows = Vec<Row>;

fn main() {
    let answer = step_one("input.txt");
    println!("Step one is : {answer}");
    let answer = step_two("input.txt");
    println!("Step two is : {answer}");
}

fn step_one(file_name: &str) -> u32 {
    let levels = load_file(file_name);
    count_safe_levels(levels)
}

fn step_two(file_name: &str) -> u32 {
    let levels = load_file(file_name);
    count_damper_levels(levels)
}

fn load_file(file_name: &str) -> Rows {
    let p = Path::new(file_name);

    let content = fs::read_to_string(p).expect("Failed to read file");

    content
        .lines()
        .into_iter()
        .map(|line| {
            line.split(" ")
                .into_iter()
                .map(|el| el.parse::<u32>().expect("Failed to parse"))
                .collect()
        })
        .collect()
}

fn count_safe_levels(rows: Rows) -> u32 {
    rows.into_iter()
        .filter_map(|entry| if is_safe_level(&entry) { Some(1) } else { None })
        .sum()
}

fn count_damper_levels(rows: Rows) -> u32 {
    rows.into_iter()
        .filter_map(|entry| {
            if is_safe_level(&entry) || (0..entry.len()).into_iter().any(|i| {
                let mut copy = entry.clone();
                copy.remove(i);
                is_safe_level(&copy)
            }) {
                Some(1)
            } else {
                None
            }
        })
        .sum()
}

fn is_safe_level(row: &Row) -> bool {
    if row.len() < 2 {
        return false;
    }

    let is_increasing = row[1] > row[0];
    let mut prev = row[0];

    for i in 1..row.len() {
        let entry = row[i];

        if prev == entry {
            return false;
        }
        if is_increasing && entry < prev {
            return false;
        }
        if !is_increasing && entry > prev {
            return false;
        }
        if prev.abs_diff(entry) > 3 {
            return false;
        }

        prev = entry;
    }
    println!("Safe : {row:?}");

    return true;
}

#[cfg(test)]
mod test {
    use crate::{step_one, step_two};

    #[test]
    pub fn test_step_one() {
        let answer = step_one("test.txt");
        assert_eq!(answer, 2);
    }

    #[test]
    pub fn test_step_two() {
        let answer = step_two("test.txt");
        assert_eq!(answer, 4);
    }
}
