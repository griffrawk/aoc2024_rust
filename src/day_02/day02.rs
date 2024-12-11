use std::fs;

#[derive(Debug)]
struct Reports {
    reports: Vec<Vec<i32>>,
}

impl Reports {
    fn new(file: &str) -> Self {
        let contents: String = fs::read_to_string(file).expect("Can't read the file");
        let mut reports: Vec<Vec<i32>> = Vec::new();
        for line in contents.lines() {
            let values = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            reports.push(values);
        }
        Self { reports }
    }
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> i32 {
    let reports = Reports::new(file);
    println!("{:?}", reports);

    2
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> i32 {
    let reports = Reports::new(file);

    0
}

#[cfg(test)]
mod tests {
    use crate::day_02::day02::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_02/day02_test.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_02/day02_data.txt");
        assert_eq!(result, 1651298);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_01/day02_test.txt");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_01/day02_data.txt");
        assert_eq!(result, 21306195);
    }
}
