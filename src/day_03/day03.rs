use std::collections::HashMap;
use itertools::Itertools;
use std::fs;
use std::cmp;

#[allow(dead_code)]
pub fn part_one_two(file: &str) -> (u32, u32) {
    let mut sum_of_part_sum: u32 = 0;
    let mut sum_of_ratio_prod: u32 = 0;
    let mut top = String::from("");
    let mut current = String::from("");
    let mut bottom = String::from("");

    let contents = fs::read_to_string(file).expect("Can't read the file");
    for line in contents.lines() {
        println!("{}", line);
        bottom = line.to_string().clone();
        if current.is_empty() {
            todo!();
        }
        // do stuff

        // shift up. probably not ideal.
        top = current.clone();
        current = bottom.clone();
    }
    println!("{} {}", sum_of_part_sum, sum_of_ratio_prod);
    (sum_of_part_sum, sum_of_ratio_prod)
}

#[cfg(test)]
mod tests {
    use crate::day_03::day03::part_one_two;

    #[test]
    fn test_part_one_two_test() {
        let result = part_one_two("src/day_03/day03_test.txt");
        assert_eq!(result, (4361, 467835));
    }

    #[test]
    fn test_part_one_two_data() {
        let result = part_one_two("src/day_03/day03_data.txt");
        assert_eq!(result, (530495, 80253814));
    }
}