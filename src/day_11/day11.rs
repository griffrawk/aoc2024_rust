use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    // Brute force method
    let mut iterations = 25;
    let mut stones: Vec<String> = fs::read_to_string(file).expect("Can't read the file")
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    while iterations > 0 {
        let mut new_stones: Vec<String> = Vec::new();
        for stone in stones {
            if stone == "0" {
                new_stones.push("1".to_string())
            } else if stone.len() % 2 == 0 {
                let (a, mut b) = stone.split_at(stone.len() / 2);
                new_stones.push(a.trim_start_matches('0').to_string());
                b = b.trim_start_matches('0');
                // trim will wipe out b if b = "000"
                if b == "" { b = "0"}
                new_stones.push(b.to_string());
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
pub fn part_two(file: &str) -> u64 {
    // Keep a map of stone to count, sum counts for the answer
    let mut iterations = 75;
    let binding = fs::read_to_string(file)
        .expect("Can't read the file");
    let mut stones: HashMap<String, u64> = binding
        .split_whitespace()
        .fold(HashMap::new(), | mut acc, stone | {
            let _ = *acc.entry(stone.to_string())
                .and_modify(| c | *c += 1)
                .or_insert(1);
            acc
        });

    while iterations > 0 {
        for (stone, count) in stones.clone() {
            if count > 0 {
                stones.entry(stone.to_string()).and_modify(|c| *c -= count);
                if stone == "0" {
                    stones.entry("1".to_string()).and_modify(|c| *c += count).or_insert(count);
                } else if stone.len() % 2 == 0 {
                    let (a, mut b) = stone.split_at(stone.len() / 2);
                    stones.entry(a.to_string()).and_modify(|c| *c += count).or_insert(count);
                    b = b.trim_start_matches('0');
                    // trim will wipe out b if b = "000"
                    if b == "" { b = "0" }
                    stones.entry(b.to_string()).and_modify(|c| *c += count).or_insert(count);
                } else {
                    let v = stone.parse::<usize>().unwrap() * 2024;
                    stones.entry(v.to_string()).and_modify(|c| *c += count).or_insert(count);
                }
            }
        }
        iterations -= 1;
    }

    stones.values().sum()
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
        assert_eq!(result, 65601038650482);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_11/day11_data.txt");
        assert_eq!(result, 231532558973909);
    }
}
