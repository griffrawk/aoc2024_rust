use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    // Brute force method
    let mut iterations = 25;
    let mut stones: Vec<u64> = fs::read_to_string(file)
        .expect("Can't read the file")
        .split_whitespace()
        .map(|s| s.parse().unwrap_or_default())
        .collect();

    while iterations > 0 {
        let mut new_stones: Vec<u64> = Vec::new();
        for stone in stones {
            if stone == 0 {
                new_stones.push(1)
            } else if stone.to_string().len() % 2 == 0 {
                let stone_string = stone.to_string();
                let (a, b) = stone_string.split_at(stone_string.len() / 2);
                new_stones.push(a.parse().unwrap());
                new_stones.push(b.parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
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
    let mut stones: HashMap<u64, u64> = fs::read_to_string(file)
        .expect("Can't read the file")
        .split_whitespace()
        .fold(HashMap::new(), |mut acc, s| {
            let _ = *acc
                .entry(s.parse().unwrap_or_default())
                .and_modify(|c| *c += 1)
                .or_insert(1);
            acc
        });

    while iterations > 0 {
        for (stone, count) in stones.clone() {
            if count > 0 {
                stones.entry(stone).and_modify(|c| *c -= count);
                let mut upd = vec![];
                if stone == 0 {
                    upd.push(1);
                } else if stone.to_string().len() % 2 == 0 {
                    let stone_string = stone.to_string();
                    let (a, b) = stone_string.split_at(stone_string.len() / 2);
                    upd.push(a.parse().unwrap());
                    upd.push(b.parse().unwrap());
                } else {
                    upd.push(stone * 2024);
                }
                for u in upd {
                    stones
                    .entry(u)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
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
