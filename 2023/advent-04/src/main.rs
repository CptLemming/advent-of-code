use std::{fs::File, io::{BufReader, BufRead}, collections::{HashSet, HashMap}, ops::RangeInclusive};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;

    let mut results = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // println!("Line {}", line);
                let split = line.split(":").collect::<Vec<&str>>();
                let cardNo = split.get(0).unwrap().replace("Card ", "").trim().parse::<u32>().unwrap();
                let games = split.get(1).unwrap().split("|").map(|entry| entry.trim()).collect::<Vec<&str>>();

                // println!("Game {} has {:?}", cardNo, games.get(1).unwrap().split(" ").collect::<Vec<&str>>());

                let winningNos = games.get(0).unwrap().split(" ").filter(|entry| !entry.is_empty()).map(|entry| entry.trim().parse::<u32>().unwrap()).collect::<HashSet<u32>>();
                let playedNos = games.get(1).unwrap().split(" ").filter(|entry| !entry.is_empty()).map(|entry| entry.trim().parse::<u32>().unwrap()).collect::<HashSet<u32>>();

                let mut score = 0;

                for playedNo in playedNos.iter() {
                    if winningNos.contains(playedNo) {
                        score += 1;
                    }
                }

                results.insert(cardNo - 1, score);
            }
            _ => {}
        }
    }

    for (key, entry) in results.iter() {
        dayOne += get_score(entry);
    }

    let dayTwo = get_no_scratchcards(&results, 0..=results.len() as u32 - 1, 0);

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

fn get_score(wins: &u32) -> u32 {
    if wins <= &1 {
        return wins.clone();
    }
    return 2u32.pow(wins - 1);
}

fn get_no_scratchcards(results: &HashMap<u32, u32>, range: RangeInclusive<u32>, depth: u32) -> u32 {
    let mut res = 0;

    for line in range {
        let entry = results.get(&line).unwrap();

        res += 1;

        if entry != &0 {
            res += get_no_scratchcards(results, line + 1..=line + entry, depth + 1);
        }
    }

    return res;
}
