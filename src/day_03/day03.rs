use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn part_one(file: &str) -> i32 {
    let contents: String = fs::read_to_string(file).expect("Can't read the file");
    let mut res = 0;
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)+").unwrap();
    for cap in re.captures_iter(&*contents) {
        let a = &cap["a"].parse::<i32>().unwrap();
        let b = &cap["b"].parse::<i32>().unwrap();
        res += a * b;
    }
    res
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> i32 {
    let contents: String = fs::read_to_string(file).expect("Can't read the file");
    let mut res = 0;
    let mut opdoflag = true;
    let re = Regex::new(r"(?<opdont>don't\(\))+|(?<opdo>do\(\))+|mul\((?<a>\d+),(?<b>\d+)\)+").unwrap();
    for cap in re.captures_iter(&*contents) {
        let opdo = &cap.name("opdo").map_or("nope", |m| m.as_str());
        let opdont = &cap.name("opdont").map_or("nope", |m| m.as_str());
        let a = &cap.name("a").map_or("0", |m| m.as_str()).parse::<i32>().unwrap();
        let b = &cap.name("b").map_or("0", |m| m.as_str()).parse::<i32>().unwrap();
        // if opdoflag changes, skip the accumulation
        if *opdo == "do()" {
            opdoflag = true
        } else if *opdont == "don't()" {
            opdoflag = false
        } else if opdoflag {
            res += a * b;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::day_03::day03::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_03/day03_test.txt");
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_03/day03_data.txt");
        assert_eq!(result, 161289189);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_03/day03_test.txt");
        assert_eq!(result, 48);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_03/day03_data.txt");
        assert_eq!(result, 83595109);
    }
}
