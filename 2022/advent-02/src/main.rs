use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // A = X = Rock = 1
    // B = Y = Paper = 2
    // C = Z = Scissors = 3
    // X = loss = 0
    // Y = draw = 3
    // Z = win = 6

    // read in file
    let file = File::open("moves.txt").expect("Failed to open file");
    // let file = File::open("test.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        match line {
            Ok(line) if !line.is_empty() => {
                let moves: Vec<&str> = line.split(" ").collect();
                let opponent = Moves::parse(moves[0]);

                // Parse as move
                // let me = Moves::parse(moves[1]);
                // let outcome = Moves::outcome(&opponent, &me);
                // score += outcome.score();
                // score += me.score();

                // Parse as condition
                let me = Outcome::parse(moves[1]);
                let me_move = Outcome::get_move(&opponent, &me);
                score += me_move.score();
                score += me.score();
            }
            _ => {}
        }
    }

    println!("Score: {}", score);
}

#[derive(Debug, PartialEq, Clone)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    pub fn parse(input: &str) -> Self {
        if input == "X" {
            return Outcome::Loss;
        } else if input == "Y" {
            return Outcome::Draw;
        }
        return Outcome::Win;
    }

    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            _ => 0,
        }
    }

    pub fn get_move(opponent: &Moves, outcome: &Self) -> Moves {
        if outcome == &Outcome::Draw {
            return opponent.clone();
        }

        if opponent == &Moves::Rock {
            return if outcome == &Outcome::Win {
                Moves::Paper
            } else {
                Moves::Scissors
            };
        } else if opponent == &Moves::Paper {
            return if outcome == &Outcome::Win {
                Moves::Scissors
            } else {
                Moves::Rock
            };
        }

        return if outcome == &Outcome::Win {
            Moves::Rock
        } else {
            Moves::Paper
        };
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    pub fn parse(input: &str) -> Self {
        if input == "A" || input == "X" {
            return Moves::Rock;
        } else if input == "B" || input == "Y" {
            return Moves::Paper;
        }
        return Moves::Scissors;
    }

    pub fn score(&self) -> u32 {
        match self {
            Moves::Rock => 1,
            Moves::Paper => 2,
            Moves::Scissors => 3,
        }
    }

    pub fn outcome(opponent: &Self, me: &Self) -> Outcome {
        if opponent == me {
            return Outcome::Draw;
        }

        if (me == &Moves::Rock && opponent == &Moves::Scissors)
            || (me == &Moves::Paper && opponent == &Moves::Rock)
            || (me == &Moves::Scissors && opponent == &Moves::Paper)
        {
            return Outcome::Win;
        }
        return Outcome::Loss;
    }
}
