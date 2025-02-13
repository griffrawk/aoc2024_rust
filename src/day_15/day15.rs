use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;
use std::ops::Add;
use colored::Colorize;
use aocutils::point::Point;

#[derive(Debug, Clone, Default)]
struct Robot {
    pos: Point<usize>,
    direction: char,
}

#[derive(Debug, Clone)]
struct Warehouse {
    robot: Robot,
    locations: HashMap<Point<usize>, char>,
    instructions: String,
}

impl Warehouse {
    fn new(file: &str) -> Self {
        let mut robot: Robot = Default::default();
        let mut locations: HashMap<Point<usize>, char> = HashMap::new();
        let mut instructions = String::new();
        let contents = fs::read_to_string(file).expect("Can't read the file");
        let mut map = true;
        for (y, line) in contents.lines().enumerate() {
            if line.is_empty() {
                map = false;
                continue;
            }
            if map {
                for (x, c) in line.chars().enumerate() {
                    match c {
                        'O' | '#' => {
                            locations.entry(Point { x, y })
                                .or_insert(c);
                        },
                        '@' => {
                            robot.pos.x = x;
                            robot.pos.y = y;
                            robot.direction = ' ';
                        },
                        _ => (),
                    }
                }
            } else {
                instructions.push_str(line);
                // instructions
            }
        }
        Warehouse { robot, locations, instructions }
    }
}

fn part_one(file: &str) -> usize {
    let warehouse = Warehouse::new(file);
    dbg!(&warehouse);

    2028
}

#[cfg(test)]
mod tests {
    use crate::day_15::day15::{part_one};

    #[test]
    fn test_part_one_basic() {
        let result = part_one("src/day_15/day15_basic.txt");
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_15/day15_test.txt");
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_15/day15_data.txt");
        assert_eq!(result, 231852216);
    }

}
