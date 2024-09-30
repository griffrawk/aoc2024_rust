use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn part_one_two(file: &str) -> (u32, u32) {
    let mut sum_up: u32 = 0;
    let mut maxima: u32 = 0;
    let contents = fs::read_to_string(file).expect("Can't read the file");
    for line in contents.lines() {
        // The parser:
        // line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        let v: Vec<&str> = line.split(':').collect();
        // v = ["Game 1"," 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]
        let game: u32 = v[0].split_whitespace().collect::<Vec<&str>>()[1]
            .parse()
            .unwrap_or_default();

        // Brief excursion into handling the Result by match:
        // let g = v[0].split_whitespace()
        //     .collect::<Vec<&str>>()[1]
        //     .parse::<u32>();
        // let game = match g {
        //     Ok(g) => g,
        //     Err(error) => {panic!("Malformed game identifier {}", error)}
        // };

        // game = 1
        let hands = v[1];
        // hands = " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        if valid_game(hands, &mut maxima) {
            sum_up += game;
        }
    }
    println!("{} {}", sum_up, maxima);
    (sum_up, maxima)
}

fn valid_game(hands: &str, maxima: &mut u32) -> bool {
    // hands = " 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let mut max_per_colour = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    let mut is_valid_game = true;
    for hand in hands.split(';') {
        // hand = " 3 blue, 4 red"
        for cube in hand.split(',') {
            // cube = " 3 blue"
            if let Some((c, colour)) = cube.split_whitespace().collect_tuple() {
                // count = "3", colour = "blue"
                let count: u32 = c.parse().unwrap_or_default();
                // assumption that the colour is valid, panic if not
                if count > limits[colour] {
                    is_valid_game = false;
                }
                max_per_colour.insert(colour, cmp::max(max_per_colour[colour], count));
            } else {
                panic!("Expected 2 elements in cube")
            }
        }
    }
    *maxima += max_per_colour.values().product::<u32>();
    is_valid_game
}

#[cfg(test)]
mod tests {
    use crate::day_02::day02::part_one_two;

    #[test]
    fn test_part_one_two_test() {
        let result = part_one_two("src/day_02/day02_test.txt");
        assert_eq!(result, (8, 2286));
    }

    #[test]
    fn test_part_one_two_data() {
        let result = part_one_two("src/day_02/day02_data.txt");
        assert_eq!(result, (2105, 72422));
    }
}
