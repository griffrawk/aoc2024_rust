use std::fs;

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let mut iterations = 25;
    let mut stones: Vec<String> = fs::read_to_string(file).expect("Can't read the file")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    while iterations > 0 {
        let mut new_stones: Vec<String> = Vec::new();
        for stone in &stones {
            if stone == "0" {
                new_stones.push("1".to_string())
            } else if stone.len() % 2 == 0 {
                let a = stone[0..stone.len() / 2].trim_start_matches('0').to_string();
                new_stones.push(a);
                let mut b = stone[stone.len() / 2..].trim_start_matches('0').to_string();
                // trim will wipe out b if b = "000"
                if b == "" { b = "0".to_string()}
                new_stones.push(b);
            } else {
                let mut v = stone.parse::<usize>().unwrap();
                v *= 2024;
                new_stones.push(v.to_string());
            }
        }
        stones = new_stones;
        iterations -= 1;
    }
    stones.len()
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
        assert_eq!(result, 194557);
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
