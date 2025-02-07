use std::cmp::min;
use std::fs;

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
    // todo - the parsing, as always...


    480
}

#[cfg(test)]
mod tests {
    use crate::day_13::day13::{claw, part_one};

    #[test]
    fn game_test() {
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
    fn test_part_one() {
        let result = part_one("src/day_13/day13_test.txt");
        assert_eq!(result, 480);
    }

}
