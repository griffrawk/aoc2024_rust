use regex::Regex;
use std::fs;

fn part_one(file: &str, max_x: i32, max_y: i32) -> i32 {
    // Calc quadrant ranges missing out the centre lines
    let left_x = 0..(max_x / 2).abs();
    let right_x = (max_x / 2).abs() + 1..max_x;
    let upper_y = 0..(max_y / 2).abs();
    let lower_y = (max_y / 2).abs() + 1..max_y;
    let mut ul = 0;
    let mut ur = 0;
    let mut ll = 0;
    let mut lr = 0;

    let contents = fs::read_to_string(file).expect("Can't read the file");
    // eg "p=0,4 v=3,-3". Named captures
    let re = Regex::new(r"p=(?<x>-?\d+),(?<y>-?\d+) v=(?<dx>-?\d+),(?<dy>-?\d+)").unwrap();
    for cap in re.captures_iter(&contents) {
        let x = &cap
            .name("x")
            .map_or("0", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let y = &cap
            .name("y")
            .map_or("0", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let dx = &cap
            .name("dx")
            .map_or("0", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let dy = &cap
            .name("dy")
            .map_or("0", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        // Do sums. A Rust gotcha. % in Rust is remainder, not modulo like in Python
        // a.rem_euclid(b) does what you'd expect instead.
        let nx = (*x + (*dx * 100)).rem_euclid(max_x);
        let ny = (*y + (*dy * 100)).rem_euclid(max_y);
        if left_x.contains(&nx) && upper_y.contains(&ny) {
            ul += 1;
        }
        if right_x.contains(&nx) && upper_y.contains(&ny) {
            ur += 1;
        }
        if left_x.contains(&nx) && lower_y.contains(&ny) {
            ll += 1;
        }
        if right_x.contains(&nx) && lower_y.contains(&ny) {
            lr += 1;
        }
    }
    ul * ur * ll * lr
}

fn part_two(file: &str) -> i64 {
    2
}

#[cfg(test)]
mod tests {
    use crate::day_14::day14::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_14/day14_test.txt", 11, 7);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_14/day14_test.txt");
        assert_eq!(result, 999);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_14/day14_data.txt", 101, 103);
        assert_eq!(result, 231852216);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_14/day14_data.txt");
        assert_eq!(result, 9999);
    }
}
