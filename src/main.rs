mod day_01;
mod day_02_2023;
mod day_03_2023;

use crate::day_03_2023::day03::part_one;

fn main() {
    println!("Hello, world!");
    let result = part_one(include_str!("day_03_2023/day03_test.txt"));
    dbg!(result);
}
