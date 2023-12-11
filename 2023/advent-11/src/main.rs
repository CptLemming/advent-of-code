use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};

type SpaceRow = Vec<String>;
type Space = Vec<SpaceRow>;
type Pairs = HashSet<Pair>;

fn main() {
    // read in file
    // let file = File::open("test.txt").expect("Failed to open file");
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut dayOne = 0;
    let mut dayTwo = 0;

    let mut space = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                space.push(parse(line));
            }
            _ => {}
        }
    }

    let dayOneSpace = expand_galaxy(space.clone(), 1);
    let dayOnePairs: Pairs = get_pairs(&dayOneSpace);
    let dayOne: u64 = get_distances(&dayOnePairs).iter().sum();

    let dayTwoSpace = expand_galaxy(space.clone(), 999_999_999);
    let dayTwoPairs: Pairs = get_pairs(&dayTwoSpace);
    let dayTwo: u64 = get_distances(&dayTwoPairs).iter().sum();

    println!("DayOne : {dayOne}");
    println!("DayTwo : {dayTwo}"); // OOM
}

fn parse(line: String) -> SpaceRow {
    return line.chars().into_iter().map(|str| str.to_string()).collect();
}

fn expand_galaxy(mut space: Space, growth: usize) -> Space {
    let mut rows: Vec<usize> = Vec::new();
    let colSize = space[0].len();

    for x in 0..space.len() {
        if space[x].iter().all(|str| str == ".") {
            rows.push(x);
        }
    }

    for (i, row) in rows.iter().enumerate() {
        for j in 0..growth {
            space.insert((i * growth) + j + row, (0..colSize).into_iter().map(|_| ".".to_string()).collect());
        }
    }

    let mut cols: Vec<usize> = Vec::new();

    for y in 0..colSize {
        if (0..space.len()).into_iter().all(|x| space[x][y] == ".") {
            cols.push(y);
        }
    }

    for (i, col) in cols.iter().enumerate() {
        for x in 0..space.len() {
            for j in 0..growth {
                space[x].insert((i * growth) + j + col, ".".to_string());
            }
        }
    }

    return space;
}

fn get_pairs(space: &Space) -> Pairs {
    let mut galaxy: Vec<(usize, usize)> = Vec::new();
    let mut pairs = HashSet::new();

    for x in 0..space.len() {
        let row = &space[x];

        for y in 0..row.len() {
            if row[y] == "#" {
                galaxy.push((x, y));
            }
        }
    }

    // println!("Galaxy# : {}", galaxy.len());
    // println!("Galaxies : {:?}", galaxy);

    for x in 0..galaxy.len() {
        for y in 0..galaxy.len() {
            if x != y {
                pairs.insert(Pair { first: galaxy[x], second: galaxy[y] });
            }
        }
    }

    // println!("Pairs : {}", pairs.len());

    return pairs;
}

fn get_distances(pairs: &Pairs) -> Vec<u64> {
    let mut lengths = Vec::new();

    for pair in pairs.iter() {
        let len = pair.len();

        // println!("Length of {:?} = {}", pair, len);
        lengths.push(len);
    }

    return lengths;
}

#[derive(Debug)]
struct Pair {
    first: (usize, usize),
    second: (usize, usize),
}

impl Pair {
    pub fn len(&self) -> u64 {
        return (((self.first.0 as i32) - (self.second.0 as i32)).abs() + ((self.first.1 as i32) - (self.second.1 as i32)).abs()) as u64;
    }
}

impl std::hash::Hash for Pair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut values = vec![self.first.0, self.first.1, self.second.0, self.second.1];
        values.sort();
        values.hash(state);
    }
}

impl Eq for Pair {
    
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.first == other.first && self.second == other.second)
        || (self.first == other.second && self.second == other.first)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::{parse, expand_galaxy, Space, Pair};

    #[test]
    pub fn test_expand_by_1() {
        let src = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

        let dst = r#"
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
"#;
        let src = src.trim().lines().into_iter().map(|line| parse(line.into())).collect::<Space>();
        let dst = dst.trim().lines().into_iter().map(|line| parse(line.into())).collect::<Space>();

        assert_eq!(expand_galaxy(src, 1).len(), dst.len());
    }

    #[test]
    pub fn test_expand_by_3() {
        let src = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

        let dst = r#"
......#............
.............#.....
#..................
...................
...................
...................
...................
............#......
.#.................
..................#
...................
...................
...................
...................
.............#.....
#......#...........
"#;
        let src = src.trim().lines().into_iter().map(|line| parse(line.into())).collect::<Space>();
        let dst = dst.trim().lines().into_iter().map(|line| parse(line.into())).collect::<Space>();

        assert_eq!(expand_galaxy(src, 3), dst);
    }

    #[test]
    pub fn test_pairs() {
        let mut pairs = HashSet::new();

        pairs.insert(Pair { first: (0, 0), second: (1, 1) });
        pairs.insert(Pair { first: (1, 1), second: (0, 0) });
        pairs.insert(Pair { first: (1, 1), second: (1, 1) });

        assert_eq!(pairs.len(), 2);
    }

    #[test]
    pub fn test_len() {
        // 1 & 7 = 15
        let pair = Pair { first: (0, 4), second: (10, 9) };
        assert_eq!(pair.len(), 15);

        // // 3 & 6 = 17
        let pair = Pair { first: (2, 0), second: (7, 12) };
        assert_eq!(pair.len(), 17);

        // 8 & 9 = 5
        let pair = Pair { first: (11, 0), second: (11, 5) };
        assert_eq!(pair.len(), 5);
    }
}
