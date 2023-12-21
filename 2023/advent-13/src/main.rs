use std::{fs::File, io::{BufReader, BufRead}, cmp};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut buffer: Vec<Vec<String>> = vec![];
    let mut index = 0;
    let mut rows = 0;
    let mut columns = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    match find_mirror(&buffer, index) {
                        Mirror::Vertical(row) => rows += row,
                        Mirror::Horizontal(col) => columns += col,
                        _ => {
                            panic!("Block {index} did not match");
                        }
                    }
                    index += 1;
                    buffer.clear();
                    continue;
                }

                buffer.push(line.chars().map(|char| char.to_string()).collect());
            }
            _ => {}
        }
    }

    if !buffer.is_empty() {
        match find_mirror(&buffer, index) {
            Mirror::Vertical(row) => rows += row,
            Mirror::Horizontal(col) => columns += col,
            _ => {
                panic!("Block {index} did not match");
            }
        }
    }

    println!("DayOneRows {rows}");
    println!("DayOneCols {columns}");
    let dayOne = columns * 100 + rows;

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

#[derive(PartialEq, Debug)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
    None
}

fn find_mirror(lines: &Vec<Vec<String>>, block: usize) -> Mirror {
    let vertical = find_vertical(lines, block);

    if vertical.is_some() {
        return Mirror::Vertical(vertical.unwrap());
    }

    let horizontal = find_horizontal(lines, block);

    if horizontal.is_some() {
        return Mirror::Horizontal(horizontal.unwrap());
    }

    Mirror::None
}

fn find_vertical(lines: &Vec<Vec<String>>, block: usize) -> Option<usize> {
    println!("Finding vertical for {block}");
    let columns = lines[0].len();

    'outer: for check_column in 0..(columns - 1) {
        // println!("Test col {check_column}");
        for index in 0..cmp::min(check_column + 1, columns - check_column - 1) {
            let x = check_column - index;
            let y = check_column + index + 1;

            // print!("Comparing {x} to {y}");
            let x = get_column(&lines, x);
            let y = get_column(&lines, y);

            if x != y {
                // println!(" = No");
                continue 'outer;
            }
            // println!(" = Yes");
        }

        // println!("Matched col {check_column}");
        return Some(check_column + 1);
    }

    None
}

fn find_horizontal(lines: &Vec<Vec<String>>, block: usize) -> Option<usize> {
    println!("Finding horizontal for {block}");
    let rows = lines.len();

    'outer: for check_row in 0..(rows - 1) {
        // println!("Test row {check_row}");
        for index in 0..cmp::min(check_row + 1, rows - check_row - 1) {
            let x = check_row - index;
            let y = check_row + index + 1;

            // print!("Comparing {x} to {y}");
            let x = &lines[x];
            let y = &lines[y];

            if x != y {
                // println!(" = No");
                continue 'outer;
            }
            // println!(" = Yes");
        }

        // println!("Matched row {check_row}");
        return Some(check_row + 1);
    }

    None
}

fn get_column(lines: &Vec<Vec<String>>, column: usize) -> Vec<String> {
    return lines.iter().map(|line| line[column].clone()).collect::<Vec<String>>();
}

#[cfg(test)]
mod test {
    use crate::{find_vertical, find_horizontal, find_mirror, Mirror};

    #[test]
    pub fn test_vertical() {
        let input = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
"#;
        let lines = input.trim().lines().map(|line| line.chars().map(|char| char.to_string()).collect()).collect();

        println!("Lines {:?}", lines);

        assert_eq!(find_vertical(&lines, 0), Some(5));
    }

    #[test]
    pub fn test_horizontal() {
        let input = r#"
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;
        let lines = input.trim().lines().map(|line| line.chars().map(|char| char.to_string()).collect()).collect();

        println!("Lines {:?}", lines);

        assert_eq!(find_horizontal(&lines, 0), Some(4));
    }

    #[test]
    pub fn test_mirror_horizontal_end() {
        let input = r#"
.##......
###.####.
##.##...#
..###..##
...##..##
#..#.##.#
..#......
.##..##..
.##..##..
"#;
        let lines = input.trim().lines().map(|line| line.chars().map(|char| char.to_string()).collect()).collect();

        println!("Lines {:?}", lines);

        assert_eq!(find_mirror(&lines, 0), Mirror::Horizontal(8));
    }

    #[test]
    pub fn test_mirror_horizontal_start() {
        let input = r#"
..####...####
..####...####
#...###...###
...##########
#.#..#..##.##
#.#.##.#..#..
.#.#..##..#..
##..###....##
#.###...##.#.
"#;
        let lines = input.trim().lines().map(|line| line.chars().map(|char| char.to_string()).collect()).collect();

        println!("Lines {:?}", lines);

        assert_eq!(find_mirror(&lines, 0), Mirror::Horizontal(1));
    }
}
