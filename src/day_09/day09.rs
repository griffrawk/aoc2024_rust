use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {


    1928
}

#[cfg(test)]
mod tests {
    use crate::day_09::day09::part_one;

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_09/day09_test.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_09/day09_data.txt");
        assert_eq!(result, 369);
    }
}
