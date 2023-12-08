use std::{fs::File, io::{BufReader, BufRead}, cmp::Ordering, collections::{HashSet, HashMap}};

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut games: Vec<Game> = vec![];

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let split = line.split(" ").collect::<Vec<&str>>();
                let hand = split.get(0).unwrap().split("").filter(|str| !str.is_empty()).map(|str| str.to_string()).collect::<Vec<String>>();

                games.push(Game::new(hand, split.get(1).unwrap().parse::<u32>().unwrap()));
            }
            _ => {}
        }
    }

    // println!("Games : {:?}", games);

    games.sort_by(|a, b| {
        if b.rank > a.rank {
            return Ordering::Less;
        } else if b.rank == a.rank {
            if get_card_rank(&b.hand[0]) > get_card_rank(&a.hand[0]) {
                return Ordering::Less;
            } else if get_card_rank(&b.hand[0]) == get_card_rank(&a.hand[0]) {
                if get_card_rank(&b.hand[1]) > get_card_rank(&a.hand[1]) {
                    return Ordering::Less;
                } else if get_card_rank(&b.hand[1]) == get_card_rank(&a.hand[1]) {
                    if get_card_rank(&b.hand[2]) > get_card_rank(&a.hand[2]) {
                        return Ordering::Less;
                    } else if get_card_rank(&b.hand[2]) == get_card_rank(&a.hand[2]) {
                        if get_card_rank(&b.hand[3]) > get_card_rank(&a.hand[3]) {
                            return Ordering::Less;
                        } else if get_card_rank(&b.hand[3]) == get_card_rank(&a.hand[3]) {
                            if get_card_rank(&b.hand[4]) > get_card_rank(&a.hand[4]) {
                                return Ordering::Less;
                            } else if get_card_rank(&b.hand[4]) == get_card_rank(&a.hand[4]) {
                                return Ordering::Equal;
                            }
                        }
                    }
                }
            }
        }
        return Ordering::Greater;
    });

    println!("Games After : {:?}", games);

    let dayOne: u32 = games.iter().enumerate().map(|(rank, game)| (rank + 1) as u32 * game.bid).sum();

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}");
}

#[derive(Debug)]
struct Game {
    hand: Vec<String>,
    bid: u32,
    rank: u8,
}

impl Game {
    pub fn new(hand: Vec<String>, bid: u32) -> Self {
        let rank = get_rank(&hand);

        return Game {
            hand,
            bid,
            rank,
        }
    }
}

fn get_rank(hand: &Vec<String>) -> u8 {
    // let mut hand_set = HashSet::new();
    // hand_set.extend(hand.clone());

    let hand_map = hand.iter().fold(HashMap::new(), |mut acc, item| {
        *acc.entry(item).or_insert(0) += 1;
        acc
    });
    let jackBonus = hand_map.get(&"J".to_string()).map(|str| str.clone()).unwrap_or(0);

    println!("Hand {:?} -> J{}", hand, jackBonus);

    let mut highest_match = hand_map.into_iter().filter(|(key, _)| key != &"J").map(|(_, entry)| entry.clone()).collect::<Vec<u8>>();
    highest_match.sort_by(|a, b| b.cmp(a));

    if highest_match.len() == 1 && highest_match[0] == 5 {
        return 6; // 5 of a kind
    } else if highest_match.len() == 2 && highest_match[0] == 4 {
        if jackBonus == 1 {
            return 6;
        }
        return 5; // four of a kind
    } else if highest_match.len() == 2 && highest_match[0] == 3 && highest_match[1] == 2 {
        return 4; // full house
    } else if highest_match.len() > 0 && highest_match[0] == 3 {
        if jackBonus == 1 {
            return 5;
        } else if jackBonus == 2 {
            return 6;
        }
        return 3; // three of a kind
    } else if highest_match.len() > 1 && highest_match[0] == 2 && highest_match[1] == 2 {
        if jackBonus == 1 {
            return 4;
        }
        return 2; // two pair
    } else if highest_match.len() > 0 && highest_match[0] == 2 {
        if jackBonus == 1 {
            return 3;
        } else if jackBonus == 2 {
            return 5;
        } else if jackBonus == 3 {
            return 6;
        }
        return 1; // one pair
    }

    if jackBonus == 5 {
        return 6;
    }
    if jackBonus > 0 {
        return get_rank(&hand.join("").replace("J", hand.iter().find(|str| str != &"J").unwrap()).split("").filter(|str| !str.is_empty()).map(|str| str.to_string()).collect::<Vec<String>>());
    }

    return 0; // high card
}

fn get_card_rank(card: &String) -> u8 {
    match card {
        str if str == "2" => 2,
        str if str == "3" => 3,
        str if str == "4" => 4,
        str if str == "5" => 5,
        str if str == "6" => 6,
        str if str == "7" => 7,
        str if str == "8" => 8,
        str if str == "9" => 9,
        str if str == "T" => 10,
        str if str == "J" => 1,
        str if str == "Q" => 12,
        str if str == "K" => 13,
        str if str == "A" => 14,
        _ => 0
    }
}

mod test {
    use crate::get_rank;

    #[test]
    fn test_five_of_a_kind() {
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "5".into(),
            "5".into(),
            "5".into(),
        ]), 6);
    }

    #[test]
    fn test_four_of_a_kind() {
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "5".into(),
            "5".into(),
            "4".into(),
        ]), 5);
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "5".into(),
            "5".into(),
            "J".into(),
        ]), 6);
    }

    #[test]
    fn test_full_house() {
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "5".into(),
            "4".into(),
            "4".into(),
        ]), 4);
    }

    #[test]
    fn test_three_of_a_kind() {
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "5".into(),
            "4".into(),
            "3".into(),
        ]), 3);
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "5".into(),
            "J".into(),
            "3".into(),
        ]), 5);
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "5".into(),
            "J".into(),
            "J".into(),
        ]), 6);
    }

    #[test]
    fn test_two_pair() {
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "2".into(),
            "2".into(),
            "4".into(),
        ]), 2);
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "2".into(),
            "2".into(),
            "J".into(),
        ]), 4);
    }

    #[test]
    fn test_one_pair() {
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "2".into(),
            "3".into(),
            "4".into(),
        ]), 1);
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "J".into(),
            "3".into(),
            "4".into(),
        ]), 3);
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "J".into(),
            "J".into(),
            "4".into(),
        ]), 5);
        assert_eq!(get_rank(&vec![
            "5".into(),
            "5".into(),
            "J".into(),
            "J".into(),
            "J".into(),
        ]), 6);
    }

    #[test]
    fn test_jacks() {
        assert_eq!(get_rank(&vec![
            "J".into(),
            "2".into(),
            "3".into(),
            "4".into(),
            "5".into(),
        ]), 1);
        assert_eq!(get_rank(&vec![
            "J".into(),
            "J".into(),
            "3".into(),
            "4".into(),
            "5".into(),
        ]), 3);
        assert_eq!(get_rank(&vec![
            "J".into(),
            "J".into(),
            "J".into(),
            "4".into(),
            "5".into(),
        ]), 5);
        assert_eq!(get_rank(&vec![
            "J".into(),
            "J".into(),
            "J".into(),
            "J".into(),
            "5".into(),
        ]), 6);
    }
}
