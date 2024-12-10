use std::fs;
use std::iter::zip;
use itertools::Itertools;

#[allow(dead_code)]
pub fn part_one(file: &str) -> i32 {
    let contents: String = fs::read_to_string(file).expect("Can't read the file");
    let mut s1: Vec<i32> = Vec::new();
    let mut s2: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let values: Vec<&str> = line.split_whitespace().collect();
        s1.push(values[0].to_string().parse::<i32>().unwrap());
        s2.push(values[1].to_string().parse::<i32>().unwrap());
    }
    let zipped = zip(s1.iter().sorted(), s2.iter().sorted()); 
    zipped.fold(0, | acc, s| acc + (s.0 - s.1).abs())
}

#[allow(dead_code)]
fn part_two(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Can't read the file");
    0
}

#[cfg(test)]
mod tests {
    use crate::day_01::day01::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_01/day01_test.txt");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_01/day01_data.txt");
        assert_eq!(result, 1651298);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("src/day_01/day01_test_part2.txt");
        // assert_eq!(result, 281);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_01/day01_data.txt");
        // assert_eq!(result, 54100);
    }
}
