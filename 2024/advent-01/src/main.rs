use std::{collections::HashMap, fs, path::Path};

type Row = (u32, u32);
type Rows = Vec<Row>;

fn main() {
  let answer = step_one("input.txt");
  println!("Step one is : {answer}");
  let answer = step_two("input.txt");
  println!("Step two is : {answer}");

}

fn step_one(file_name: &str) -> u32 {
  let entries = load_file(file_name).expect("Not to fail");
  let sorted = sort_entries(entries);
  get_difference(sorted)
}

fn step_two(file_name: &str) -> u32 {
  let entries = load_file(file_name).expect("Not to fail");
  get_score(entries)
}

fn load_file(file_name: &str) -> Result<Rows, String> {
  let p = Path::new(file_name);

  let content = fs::read_to_string(p).expect("Failed to read file");
  let mut out = Vec::new();

  for line in content.lines() {
    let row = line.split(" ").map(|item| item.trim()).filter(|item| !item.is_empty()).collect::<Vec<&str>>();

    if row.len() == 2 {
      out.push((row[0].parse::<u32>().expect("Parse 0 failed"), row[1].parse::<u32>().expect("Parse 1 failed")));
    }
  }

  Ok(out)
}

fn sort_entries(rows: Rows) -> Rows {
  let mut left = Vec::new();
  let mut right = Vec::new();

  for row in rows {
    left.push(row.0);
    right.push(row.1);
  }

  left.sort();
  right.sort();

  (0..left.len()).into_iter().map(|i| (left[i], right[i])).collect()
}

fn get_difference(rows: Rows) -> u32 {
  let mut out = 0;

  for row in rows {
    out += row.0.abs_diff(row.1);
  }

  out
}

fn get_score(rows: Rows) -> u32 {
  let mut out = 0;

  let mut left = Vec::new();
  let mut right = Vec::new();

  for row in rows {
    left.push(row.0);
    right.push(row.1);
  }

  let mut right_sum: HashMap<u32, u32> = HashMap::new();

  for row in right.iter() {
    *right_sum.entry(row.to_owned()).or_default() += 1;
  }

  for row in left {
    if let Some(val) = right_sum.get(&row) {
      out += row * val;
    }
  }

  out
}

#[cfg(test)]
mod test {
  use crate::{step_one, step_two};

  #[test]
  pub fn test_step_one() {
    let answer = step_one("test.txt");
    assert_eq!(answer, 11);
  }

  #[test]
  pub fn test_step_two() {
    let answer = step_two("test.txt");
    assert_eq!(answer, 31);
  }
}
