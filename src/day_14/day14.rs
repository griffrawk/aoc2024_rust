use regex::Regex;
use std::fs;
// use colored::Colorize;
use aocutils::point::Point;

#[derive(Debug, Clone)]
struct Robot {
    pos: Point<i32>,
    dx: i32,
    dy: i32,
}

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
        let nx = (x + (dx * 100)).rem_euclid(max_x);
        let ny = (y + (dy * 100)).rem_euclid(max_y);
        if left_x.contains(&nx) && upper_y.contains(&ny) {
            ul += 1;
        } else if right_x.contains(&nx) && upper_y.contains(&ny) {
            ur += 1;
        } else if left_x.contains(&nx) && lower_y.contains(&ny) {
            ll += 1;
        } else if right_x.contains(&nx) && lower_y.contains(&ny) {
            lr += 1;
        }
    }
    ul * ur * ll * lr
}

fn part_two(file: &str, max_x: i32, max_y: i32) -> i32 {
    // Calc quadrant ranges missing out the centre lines
    let left_x = 0..(max_x / 2).abs();
    let right_x = (max_x / 2).abs() + 1..max_x;
    let upper_y = 0..(max_y / 2).abs();
    let lower_y = (max_y / 2).abs() + 1..max_y;

    let mut min_q = i32::MAX;
    let mut t_at_min = 0;

    let mut hall: Vec<Robot> = Vec::new();

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
        // push robots on a Vec
        hall.push(Robot { pos: Point { x: *x, y: *y }, dx: *dx, dy: *dy });
    }

    for t in 0..8160 {
        // let mut plot = vec![vec![' '; max_x as usize]; max_y as usize];
        let mut ul = 0;
        let mut ur = 0;
        let mut ll = 0;
        let mut lr = 0;
        for robot in hall.clone() {
            let pos = robot_pos(robot, max_x, max_y, t);
            // plot[pos.y as usize][pos.x as usize] = '*';

            if left_x.contains(&pos.x) && upper_y.contains(&pos.y) {
                ul += 1;
            } else if right_x.contains(&pos.x) && upper_y.contains(&pos.y) {
                ur += 1;
            } else if left_x.contains(&pos.x) && lower_y.contains(&pos.y) {
                ll += 1;
            } else if right_x.contains(&pos.x) && lower_y.contains(&pos.y) {
                lr += 1;
            }
        }
        // plot it anyway
        // let bar = "-".repeat(max_y as usize).blue();
        // println!("{}", bar);
        // for (y, line) in plot.iter().enumerate() {
        //     print!("{}", "|".blue());
        //     for (x, r ) in line.iter().enumerate() {
        //         if x as i32 == (max_x / 2).abs() || y as i32 == (max_y / 2).abs() {
        //             centre line
        //             print!("{}", r.to_string().red());
        //         } else {
        //             print!("{}", r.to_string().bright_green());
        //         }
        //     }
        //     println!("{}", "|".blue());
        // }
        // println!("{}", bar);

        let m = ul * ur * ll * lr;
        // println!("At t = {}, safety factor = {}\n\n", t, m);

        if m < min_q {
            min_q = m;
            t_at_min = t;
        }
    }
    dbg!(min_q, t_at_min);

    t_at_min
}

fn robot_pos(robot: Robot, max_x: i32, max_y: i32, t: i32) -> Point<i32> {
    // Robot's position after t seconds
    Point { x: (robot.pos.x + (robot.dx * t)).rem_euclid(max_x),
            y: (robot.pos.y + (robot.dy * t)).rem_euclid(max_y) }
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
    fn test_part_one_data() {
        let result = part_one("src/day_14/day14_data.txt", 101, 103);
        assert_eq!(result, 231852216);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_14/day14_data.txt", 101, 103);
        assert_eq!(result, 8159);
    }
}
