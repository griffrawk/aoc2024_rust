use std::collections::HashMap;
use itertools::Itertools;
use std::fs;

#[allow(dead_code)]
pub fn part_one(file: &str) -> u32 {
    let mut sum_up = 0;
    let contents = fs::read_to_string(file).expect("Can't read the file");
    for line in contents.lines() {
        let v: Vec<&str> = line.split(':').collect();
        let game = v[0].split_whitespace()
            .collect::<Vec<_>>()[1]
            .parse::<u32>().unwrap();
        let hands = v[1];
        if valid_game(hands) {
            sum_up += game;
        }
    }
    println!("{}", sum_up);
    sum_up
}

fn valid_game(hands: &str) -> bool {
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    for hand in hands.split(';') {
        for cube in hand.split(',') {
            if let Some((count , colour)) = cube.split_whitespace().collect_tuple() {
                if count.parse::<u32>().unwrap() > limits[colour] {
                    return false
                }
            } else {
                panic!("Expected 2 elements in cube")
            }
        }
    }
    true
}


#[allow(dead_code)]
fn part_two(file: &str) -> u32 {
    let sum_up = 0;
    println!("{}", sum_up);
    sum_up
}

#[cfg(test)]
mod tests {
    use crate::day_02::day02::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_02/day02_test.txt");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_02/day02_data.txt");
        assert_eq!(result, 2105);
    }
}