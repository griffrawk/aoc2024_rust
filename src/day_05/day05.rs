use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let mut res = 0;
    let contents = fs::read_to_string(file).expect("Can't read the file");
    let mut rules: HashMap<String, usize> = HashMap::new();
    let mut updates: Vec<Vec<&str>> = Vec::new();

    for line in contents.lines() {
        if line.contains("|") {
            rules.insert(line.to_string(), 0);
        }
        if line.contains(",") {
            updates.push(line.split(",").collect());
        }
    }

    'nextupdate: for update in updates {
        for (idx, page) in update.iter().enumerate() {
            for backref in 0..update.len() {
                if backref < idx {
                    let key = format!("{}|{}", update[backref], page);
                    if !rules.contains_key(&key) { continue 'nextupdate}
                } else if backref > idx {
                    let key = format!("{}|{}", page, update[backref]);
                    if !rules.contains_key(&key) { continue 'nextupdate}
                }
            }
        }
        // inc res with middle value
        res += update[(update.len() as i32 / 2) as usize]
            .parse::<usize>().unwrap();
    }
    res
}

pub fn part_two(file: &str) -> i32 {
    let mut res = 0;
    let contents = fs::read_to_string(file).expect("Can't read the file");
    let mut wordsearch: Vec<String> = Vec::new();
    for line in contents.lines() {
        wordsearch.push(line.to_string());
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::day_05::day05::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_05/day05_test.txt");
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_05/day05_data.txt");
        assert_eq!(result, 4569);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_05/day05_test.txt");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_05/day05_data.txt");
        assert_eq!(result, 1948);
    }
}
