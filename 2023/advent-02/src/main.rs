use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let red_quota = 12;
    let green_quota = 13;
    let blue_quota = 14;

    let mut dayOne = 0;
    let mut dayTwo = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Sample: Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red

                let game = parse_game(line);

                // println!("Game: {:?}", game);
                if game.is_valid(red_quota, green_quota, blue_quota) {
                    // println!("Is valid : {}", game.id);
                    dayOne += game.id;
                }

                let power = game.get_power();
                // println!("Power: {power}");

                dayTwo += power;
            }
            _ => {}
        }
    }

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

fn parse_game(line: String) -> Game {
    let split = line.split(":").collect::<Vec<&str>>();
    let game_no = split.get(0).unwrap().replace("Game ", "").parse::<u32>().unwrap();

    let rounds = split.get(1).unwrap().split(";").map(|items| {
        let mut round = Round::default();
        for item in items.split(",").into_iter() {
            match item {
                s if s.contains("red") => { round.red = s.replace("red", "").trim().parse::<u32>().unwrap(); }
                s if s.contains("green") => { round.green = s.replace("green", "").trim().parse::<u32>().unwrap(); }
                s if s.contains("blue") => { round.blue = s.replace("blue", "").trim().parse::<u32>().unwrap(); }
                _ => {}
            }
        }

        return round;
    }).collect::<Vec<Round>>();

    return Game {
        id: game_no,
        rounds,
    }
}

#[derive(Default, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn is_valid(&self, red: u32, green: u32, blue: u32) -> bool {
        return self.rounds.iter().all(|round| round.red <= red && round.green <= green && round.blue <= blue);
    }

    pub fn get_power(&self) -> u32 {
        let round = self.rounds.iter().fold(Round::default(), |mut acc, round| {
            if round.red > acc.red {
                acc.red = round.red;
            }
            if round.green > acc.green {
                acc.green = round.green;
            }
            if round.blue > acc.blue {
                acc.blue = round.blue;
            }
            return acc;
        });

        return round.red * round.green * round.blue;
    }
}