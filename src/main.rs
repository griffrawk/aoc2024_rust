mod day_01;
mod day_02;
mod day_03;

use crate::day_03::day03::part_one;

fn main() {
    println!("Hello, world!");
    let result = part_one(include_str!("day_03/day03_test.txt"));
    dbg!(result);
}
