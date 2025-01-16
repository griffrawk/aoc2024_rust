use std::fs;

// Part 1 structs & impl
#[derive(Debug)]
struct Disk {
    // blocks: Vec<Option<usize>>,
}

impl Disk {
    fn new(file: &str) -> Disk {
        // let mut blocks = Vec::new();
        for (map_count, map_block) in fs::read_to_string(file)
            .expect("Can't read the file")
            .chars()
            .enumerate()
        {
            // do something
        }
        Disk { }
    }
}

// mains
#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {

    999
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> usize {
    999
}

#[cfg(test)]
mod tests {
    use crate::day_10::day10::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_10/day10_test.txt");
        assert_eq!(result, 999);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_10/day10_data.txt");
        assert_eq!(result, 999);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_10/day10_test.txt");
        assert_eq!(result, 999);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_10/day10_data.txt");
        assert_eq!(result, 999);
    }
}
