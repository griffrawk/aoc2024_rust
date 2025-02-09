use std::cmp::min;
use std::fs;
use regex::Regex;

#[allow(dead_code)]
fn claw(xa: i32, xb: i32, ya: i32, yb: i32, prize_x: i32, prize_y: i32) -> i32 {
    // As pressing B is cheaper, maximise that at start. How many B presses will
    // get the claw *nearly* there?
    // '...no more than 100 times to win a prize. How else would someone be expected to play?'
    // plus 1 to offset initial decrement
    let mut a_presses = 0;
    let mut b_presses = min(100, (prize_x / xb).abs()) + 1;
    let mut px = 0;
    let mut py = 0;

    while px != prize_x || py != prize_y {
        b_presses -= 1;
        if b_presses == 0 {
            // No more B presses left, so fail, set presses to 0 so returned cost = 0
            a_presses = 0;
            break;
        }
        // Calculate how many A presses will fill the gap between those provided by
        // B presses and the prize
        a_presses = ((prize_x - (b_presses * xb)) / xa).abs();
        // Are we on the prize?
        px = a_presses * xa + b_presses * xb;
        // As X and Y prizes are both dependent on the same number of A & B presses,
        // then Y just follows. No need for separate calculation
        py = a_presses * ya + b_presses * yb;
    }
    // return token cost
    a_presses * 3 + b_presses
}

fn part_one(file: &str) -> i32 {
    let mut res = 0;
    let mut xa = 0;
    let mut ya = 0;
    let mut xb = 0;
    let mut yb = 0;
    let mut prize_x = 0;
    let mut prize_y = 0;

    // todo -
    let contents = fs::read_to_string(file).expect("Can't read the file");
    for line in contents.lines() {
        if line.contains("Button A:") {
            let i: Vec<&str> = line.split(|c| "+,".contains(c)).collect();
            xa = i[1].parse().unwrap();
            ya = i[3].parse().unwrap();
            continue;
        }
        if line.contains("Button B:") {
            let i: Vec<&str> = line.split(|c| "+,".contains(c)).collect();
            xb = i[1].parse().unwrap();
            yb = i[3].parse().unwrap();
            continue;
        }
        if line.contains("Prize:") {
            let i: Vec<&str> = line.split(|c| "=,".contains(c)).collect();
            prize_x = i[1].parse().unwrap();
            prize_y = i[3].parse().unwrap();
            res += claw(xa, xb, ya, yb, prize_x, prize_y);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::day_13::day13::{claw, part_one};

    #[test]
    fn claw_test() {
        // 280
        let xa = 94;
        let xb = 22;
        let prize_x: i32 = 8400;
        let ya = 34;
        let yb = 67;
        let prize_y = 5400;
        assert_eq!(claw(xa, xb, ya, yb, prize_x, prize_y), 280);

        // 0
        let xa = 26;
        let xb = 67;
        let prize_x: i32 = 12748;
        let ya = 66;
        let yb = 21;
        let prize_y = 12176;
        assert_eq!(claw(xa, xb, ya, yb, prize_x, prize_y), 0);

        // 200
        let xa = 17;
        let xb = 84;
        let prize_x: i32 = 7870;
        let ya = 86;
        let yb = 37;
        let prize_y = 6450;
        assert_eq!(claw(xa, xb, ya, yb, prize_x, prize_y), 200);

        // 0
        let xa = 69;
        let xb = 27;
        let prize_x: i32 = 18641;
        let ya = 23;
        let yb = 71;
        let prize_y = 10279;
        assert_eq!(claw(xa, xb, ya, yb, prize_x, prize_y), 0);
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_13/day13_test.txt");
        assert_eq!(result, 480);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_13/day13_data.txt");
        assert_eq!(result, 29598);
    }

}
