use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    // let file = File::open("test2.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut start_pos = (0, 0);
    let mut maze: Vec<Vec<String>> = vec![];

    for (x, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                maze.push(line.chars().enumerate().map(|(y, str)| {
                    if str.to_string() == "S" {
                        start_pos = (x, y);
                    }
                    return str.to_string();
                }).collect());
            }
            _ => {}
        }
    }

    println!("Pos : {start_pos:?}");
    println!("Maze : {maze:?}");

    let mut steps = 0;
    let mut prev = start_pos;
    let mut current = start_pos;

    loop {
        let cell = &maze[current.0][current.1];

        if cell == "S" {
            // find possible moves
            let mut moves: Vec<(usize, usize)> = vec![];

            // North
            if current.0 != 0 {
                match maze.get(current.0 - 1).map(|row| row.get(current.1)) {
                    Some(Some(cell)) if cell == "|" || cell == "7" || cell == "F" => {
                        moves.push((current.0 - 1, current.1));
                    }
                    _ => {}
                }
            }

            // East
            match maze.get(current.0).map(|row| row.get(current.1 + 1)) {
                Some(Some(cell)) if cell == "-" || cell == "L" || cell == "F" => {
                    moves.push((current.0, current.1 + 1));
                }
                _ => {}
            }

            // South
            match maze.get(current.0 + 1).map(|row| row.get(current.1)) {
                Some(Some(cell)) if cell == "|" || cell == "L" || cell == "J" => {
                    moves.push((current.0 + 1, current.1));
                }
                _ => {}
            }

            // West
            if current.1 != 0 {
                match maze.get(current.0).map(|row| row.get(current.1 - 1)) {
                    Some(Some(cell)) if cell == "-" || cell == "J" || cell == "7" => {
                        moves.push((current.0, current.1 - 1));
                    }
                    _ => {}
                }
            }

            println!("Starter moves : {moves:?}");

            prev = current;
            current = moves[0];
        } else {
            // find possible moves
            let mut moves: Vec<(usize, usize)> = vec![];

            match cell {
                s if s == "|" => {
                    moves.push((current.0 - 1, current.1)); // N
                    moves.push((current.0 + 1, current.1)); // S
                }
                s if s == "-" => {
                    moves.push((current.0, current.1 + 1)); // E
                    moves.push((current.0, current.1 - 1)); // W
                }
                s if s == "L" => {
                    moves.push((current.0 - 1, current.1)); // N
                    moves.push((current.0, current.1 + 1)); // E
                }
                s if s == "J" => {
                    moves.push((current.0 - 1, current.1)); // N
                    moves.push((current.0, current.1 - 1)); // W
                }
                s if s == "7" => {
                    moves.push((current.0 + 1, current.1)); // S
                    moves.push((current.0, current.1 - 1)); // W
                }
                s if s == "F" => {
                    moves.push((current.0 + 1, current.1)); // S
                    moves.push((current.0, current.1 + 1)); // E
                }
                _ => {}
            }

            println!("Moves : {moves:?}");
            let next = moves.iter()
                .filter(|pos| maze[pos.0][pos.1] != ".")
                .find(|pos| pos != &&prev)
                .unwrap();
            println!("Next move : {next:?}");
            prev = current;
            current = next.clone();
        }

        steps += 1;

        if current == start_pos {
            break;
        }
    }

    dayOne = steps / 2;

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

fn filter_maze(maze: &Vec<Vec<String>>, current: &(usize, usize), possible: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    return possible.iter().filter(|pos| {
        let cell = &maze[current.0][current.1];

        return false;
    }).map(|pos| pos.clone()).collect();
}
