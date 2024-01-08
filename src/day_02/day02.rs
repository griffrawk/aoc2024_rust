use std::fs;

#[allow(dead_code)]
pub fn part_one(file: &str) -> u32 {
    // read a file per line, for each line filter for digits and concatenate them
    // sum the first and last digit of each line as a new number
    // eg '1234' -> sum_up += 14

    let contents = fs::read_to_string(file).expect("Can't read the file");
    let mut sum_up = 0;
    for line in contents.lines() {
        let numbs: Vec<u32> = line.chars()
            .filter(|a| a.is_digit(10))
            .map(|a| a.to_string().parse::<u32>().unwrap())
            .collect();

        println!("{:?}", numbs);
        sum_up += numbs.first()
                .unwrap_or(&0) * 10
            + numbs.last()
                .unwrap_or(&0);
    }
    println!("{}", sum_up);
    sum_up
}

#[allow(dead_code)]
fn part_two(file: &str) -> u32 {
    let contents = fs::read_to_string(file).expect("Can't read the file");
    let mut sum_up = 0;
    let numbers = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in contents.lines() {
        let mut a_copy = String::from(line);
        for i in 0..=9 {
            // Replace a number in words into a worddigitword eg 'eight' becomes 'eight8eight'
            // This avoids clobbering occurrences where words overlap by a letter
            // eg. 'twone' -> 'two2twone1one'. Thus the filter section finds both numeric digits properly.
            let replacement = numbers[i].to_string() + &i.to_string() + &numbers[i];
            a_copy = a_copy.replace(numbers[i], replacement.as_str());
        }
        println!("{} {}", line, a_copy);

        // Now filter for the numeric digits and make a Vec
        let numbs: Vec<u32> = a_copy.chars()
            .filter(|a| a.is_digit(10))
            .map(|a| a.to_string().parse::<u32>().unwrap())
            .collect();

        println!("{:?}", numbs);
        // Pick first and last digits (if present, else 0) and sum
        sum_up += numbs.first()
            .unwrap_or(&0) * 10
            + numbs.last()
            .unwrap_or(&0);

    }
    println!("{}", sum_up);
    sum_up
}

#[cfg(test)]
mod tests {
    use crate::day_02::day02::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_02/day02_test.txt");
        assert_eq!(result, 142);
    }
    
    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_02/day02_data.txt");
        assert_eq!(result, 54877);
    }

    #[test]
    fn test_part_two() {
        let result = part_two("src/day_02/day02_test_part2.txt");
        assert_eq!(result, 281);
    }
    
    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_02/day02_data.txt");
        assert_eq!(result, 54100);
    }
}
