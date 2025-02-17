#[cfg(test)]
mod tests {

    use std::cmp::max;
    use std::fs;

    /// Take Vec<String> and return a String consisting of contents read diagonally
    /// in NE direction (45 degrees clockwise from vertical if you like...)
    fn stringify_rot45(input: &Vec<String>) -> String {
        let max_x = input[0].len();
        let max_y = input.len();
        let n = max(max_y, max_x);
        let mut res = String::new();
        // 1st half
        for base_y in 0..n {
            let mut y = base_y;
            for x in 0..=base_y {
                // need to check bounds and ignore
                if x < max_x && y < max_y {
                    res.push_str(letter(input, x, y));
                }
                if y > 0 {
                    y -= 1
                }
            }
            // delimit the line to avoid wraparound searching when as a flat String
            res.push_str("|");
        }
        // 2nd half
        for base_x in 1..n {
            let mut x = base_x;
            for y in (base_x..n).rev() {
                // need to check bounds and ignore
                if x < max_x && y < max_y {
                    res.push_str(letter(input, x, y));
                }
                x += 1;
            }
            // delimit the line to avoid wraparound searching when as a flat String
            res.push_str("|");
        }
        res
    }

    fn rot90(input: &Vec<String>) -> Vec<String> {
        let m = input[0].len();
        let n = input.len();
        let mut res: Vec<String> = Vec::new();
        for x in (0..m).rev() {
            let mut line = String::new();
            for y in 0..n {
                line.push_str(letter(input, x, y));
            }
            // delimit the line to avoid wraparound searching when as a flat String
            line.push_str("|");
            res.push(line);
        }
        res
    }

    fn letter(input: &Vec<String>, x: usize, y: usize) -> &str {
        input[y].get(x..x + 1).unwrap()
    }

    pub fn part_one(file: &str) -> usize {
        let mut res = 0;
        let contents = fs::read_to_string(file).expect("Can't read the file");
        let mut wordsearch: Vec<String> = Vec::new();
        for line in contents.lines() {
            wordsearch.push(line.to_string());
        }
        // delimit the rot0 line to avoid wraparound searching when as a flat String
        // done beforehand in the rot45 & rot90 methods, so not needed again
        let w0 = wordsearch.join("|");
        let w45 = stringify_rot45(&wordsearch);
        let w90 = rot90(&wordsearch).join("");
        let w135 = stringify_rot45(&rot90(&wordsearch));

        res += w0.matches("XMAS").collect::<Vec<_>>().len();
        res += w0.matches("SAMX").collect::<Vec<_>>().len();
        res += w45.matches("XMAS").collect::<Vec<_>>().len();
        res += w45.matches("SAMX").collect::<Vec<_>>().len();
        res += w90.matches("XMAS").collect::<Vec<_>>().len();
        res += w90.matches("SAMX").collect::<Vec<_>>().len();
        res += w135.matches("XMAS").collect::<Vec<_>>().len();
        res += w135.matches("SAMX").collect::<Vec<_>>().len();
        res
    }
    pub fn part_two(file: &str) -> i32 {
        let mut res = 0;
        let contents = fs::read_to_string(file).expect("Can't read the file");
        let mut wordsearch: Vec<String> = Vec::new();
        for line in contents.lines() {
            wordsearch.push(line.to_string());
        }
        let m = wordsearch[0].len();
        let n = wordsearch.len();

        for x in 1..m - 1 {
            for y in 1..n - 1 {
                if letter(&wordsearch, x, y) == "A" {
                    if ((letter(&wordsearch, x - 1, y - 1) == "M"
                        && letter(&wordsearch, x + 1, y + 1) == "S")
                        || (letter(&wordsearch, x - 1, y - 1) == "S"
                            && letter(&wordsearch, x + 1, y + 1) == "M"))
                        && ((letter(&wordsearch, x + 1, y - 1) == "M"
                            && letter(&wordsearch, x - 1, y + 1) == "S")
                            || (letter(&wordsearch, x + 1, y - 1) == "S"
                                && letter(&wordsearch, x - 1, y + 1) == "M"))
                    {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_04/day04_test.txt");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_04/day04_data.txt");
        assert_eq!(result, 2599);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_04/day04_test.txt");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_04/day04_data.txt");
        assert_eq!(result, 1948);
    }
}
