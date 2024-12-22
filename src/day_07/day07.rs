use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

fn bin_count(bits: usize) -> Vec<String> {
    let z = (bits as f32).exp2() as usize;
    (0..z).into_iter().map(|n| {
        format!("{:0b$b}", n, b = bits)
    }).collect()
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let contents: String = fs::read_to_string(file).expect("Can't read the file");


    0
}

pub fn part_two(file: &str) -> usize {

    0
}

#[cfg(test)]
mod tests {
    use crate::day_07::day07::{bin_count, part_one };

    #[test]
    fn test_bin_count() {
        assert_eq!(bin_count(3), vec![
            "000".to_string(),
            "001".to_string(),
            "010".to_string(),
            "011".to_string(),
            "100".to_string(),
            "101".to_string(),
            "110".to_string(),
            "111".to_string(),
        ]);
        assert_eq!(bin_count(2), vec![
            "00".to_string(),
            "01".to_string(),
            "10".to_string(),
            "11".to_string(),
        ])
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_07/day07_test.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_07/day07_data.txt");
        assert_eq!(result, 5095);
    }

}
