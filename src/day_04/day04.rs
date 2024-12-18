use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
// use regex::Regex;

#[allow(dead_code)]
pub fn part_one(file: &str) -> i32 {
    let mut res = 0;
    let contents = fs::read_to_string(file).expect("Can't read the file");
    let mut wordsearch: Vec<String> = Vec::new();
    for line in contents.lines() {
        wordsearch.push(line.to_string());

    }
    dbg!(&wordsearch);

    res
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> i32 {

    0
}

#[cfg(test)]
mod tests {
    use crate::day_04::day04::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_04/day04_test.txt");
        // assert_eq!(result, 18);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_04/day04_data.txt");
        assert_eq!(result, 161289189);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_04/day04_test.txt");
        assert_eq!(result, 48);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_04/day04_data.txt");
        assert_eq!(result, 83595109);
    }
}
