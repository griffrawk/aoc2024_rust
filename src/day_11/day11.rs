use std::collections::{HashMap, HashSet};
use std::fs;

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let iterations = 25;
    let mut stones: Vec<String> = fs::read_to_string(file).expect("Can't read the file")
        .split_whitespace()
        .collect();
    dbg!(&stones);

    55312
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> usize {
    81
}

#[cfg(test)]
mod tests {
    use crate::day_11::day11::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_11/day11_test.txt");
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_11/day11_data.txt");
        assert_eq!(result, 430);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_11/day11_test.txt");
        assert_eq!(result, 81);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_10/day10_data.txt");
        assert_eq!(result, 928);
    }
}
