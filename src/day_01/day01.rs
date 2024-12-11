use itertools::Itertools;
use std::fs;
use std::iter::zip;

struct Locations {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

impl Locations {
    fn new(file: &str) -> Self {
        let contents: String = fs::read_to_string(file).expect("Can't read the file");
        let mut s1: Vec<i32> = Vec::new();
        let mut s2: Vec<i32> = Vec::new();
        for line in contents.lines() {
            let values: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            s1.push(values[0]);
            s2.push(values[1]);
        }
        Self { s1, s2 }
    }
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> i32 {
    let locations = Locations::new(file);
    let zipped = zip(locations.s1.iter().sorted(), locations.s2.iter().sorted());
    zipped.fold(0, |acc, s| acc + (s.0 - s.1).abs())
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> i32 {
    let locations = Locations::new(file);
    locations.s1.iter().fold(0, |acc, s| {
        acc + s * locations.s2.iter().filter(|&n| *n == *s).count() as i32
    })
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
    fn test_part_two_test() {
        let result = part_two("src/day_01/day01_test.txt");
        assert_eq!(result, 31);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_01/day01_data.txt");
        assert_eq!(result, 21306195);
    }
}
