#[cfg(test)]
mod tests {

    use std::collections::HashSet;
    use std::fs;

    fn part_one_two(file: &str) -> (usize, usize) {
        let mut res = 0;
        let mut corres = 0;
        let contents = fs::read_to_string(file).expect("Can't read the file");
        let mut rules: HashSet<String> = HashSet::new();
        let mut updates: Vec<Vec<&str>> = Vec::new();

        for line in contents.lines() {
            if line.contains("|") {
                rules.insert(line.trim().to_string());
            }
            if line.contains(",") {
                updates.push(line.trim().split(",").collect());
            }
        }

        'nextupdate: for mut update in updates {
            let mut corrected = false;
            for idx in 0..update.len() {
                for fwdref in idx + 1..update.len() {
                    // For each page, check current / next page for valid rule
                    // previous / current is already passed by getting that far so no
                    // need to recheck
                    let key = format!("{}|{}", update[idx], update[fwdref]);
                    if !rules.contains(&key) {
                        // Try the key the other way round (part 2)
                        let key = format!("{}|{}", update[fwdref], update[idx]);
                        if rules.contains(&key) {
                            corrected = true;
                            // Need to swap the corrected pages, so later checks work
                            // and we pick the correct middle
                            let temp = update[fwdref];
                            update[fwdref] = update[idx];
                            update[idx] = temp;
                        } else {
                            // Skip update if rule not found either way round
                            continue 'nextupdate;
                        }
                    }
                }
            }
            // Valid update - inc res with middle value
            let middle = (update.len() as i32 / 2) as usize;
            if corrected {
                corres += update[middle].parse::<usize>().unwrap();
            } else {
                res += update[middle].parse::<usize>().unwrap();
            }
        }
        (res, corres)
    }

    #[test]
    fn test_part_one_two_test() {
        let result = part_one_two("src/bin/day05/day05_test.txt");
        assert_eq!(result, (143, 123));
    }

    #[test]
    fn test_part_one_two_data() {
        let result = part_one_two("src/bin/day05/day05_data.txt");
        assert_eq!(result, (4569, 6456));
    }
}

fn main() {}
