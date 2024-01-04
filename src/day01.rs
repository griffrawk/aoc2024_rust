use std::fs;

#[allow(dead_code)]
fn part_one() -> u32 {
    // read a file per line, for each line filter for digits and concatenate them
    // sum the first and last digit of each line as a new number
    // eg '1234' -> sumup += 14

    // so the first challenge in any language is reading files. not sure if ive
    // done that in rust before

    let contents = fs::read_to_string("src/day01_test.txt").expect("Can't read the file");
    let mut sum_up = 0;
    println!("{}", contents);
    for line in contents.lines() {
        println!("a line {}", line);
        let numbs: Vec<char> = line.chars()
            .filter(|a| a.is_digit(10))
            .collect();
        println!("{:?}", numbs);
        sum_up += numbs.first()
                .unwrap_or(&'0')
                .to_digit(10)
                .unwrap_or(0) * 10
            + numbs.last()
                .unwrap_or(&'0')
                .to_digit(10)
                .unwrap_or(0);
    }
    println!("{}", sum_up);
    sum_up
}

#[cfg(test)]
mod tests {
    use crate::day01::part_one;

    #[test]
    fn test_part_one() {
        let result = part_one();
        assert_eq!(result, 142);
    }
}
