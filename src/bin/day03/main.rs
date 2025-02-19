#[cfg(test)]
mod tests {

    use regex::Regex;
    use std::fs;

    pub fn part_one(file: &str) -> i32 {
        let contents: String = fs::read_to_string(file).expect("Can't read the file");
        let mut res = 0;
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        for (_, [a, b]) in re.captures_iter(&contents).map(|c| c.extract()) {
            res += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
        res
    }

    pub fn part_two(file: &str) -> i32 {
        let contents: String = fs::read_to_string(file).expect("Can't read the file");
        let mut res = 0;
        let mut opdoflag = true;
        // Here, I have to use named captures, as I can't use the .extract() into tuple method as above. That's
        // because all 4 captures may not be present. don't() or do() or mul()
        let re = Regex::new(r"(?<opdont>don't\(\))+|(?<opdo>do\(\))+|mul\((?<a>\d+),(?<b>\d+)\)+")
            .unwrap();
        for caps in re.captures_iter(&contents) {
            let opdo = &caps.name("opdo").map_or("nope", |m| m.as_str());
            let opdont = &caps.name("opdont").map_or("nope", |m| m.as_str());
            let a = &caps
                .name("a")
                .map_or("0", |m| m.as_str())
                .parse::<i32>()
                .unwrap();
            let b = &caps
                .name("b")
                .map_or("0", |m| m.as_str())
                .parse::<i32>()
                .unwrap();
            // if opdoflag changes, skip the accumulation
            if *opdo == "do()" {
                opdoflag = true
            } else if *opdont == "don't()" {
                opdoflag = false
            } else if opdoflag {
                res += a * b;
            }
        }
        res
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/bin/day03/day03_test.txt");
        assert_eq!(result, 161);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/bin/day03/day03_data.txt");
        assert_eq!(result, 161289189);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/bin/day03/day03_test.txt");
        assert_eq!(result, 48);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/bin/day03/day03_data.txt");
        assert_eq!(result, 83595109);
    }
}

fn main() {}
