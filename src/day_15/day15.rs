use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
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

    fn move_robot(&mut self) {
        for instruction in self.instructions.clone().chars() {
            // calc proposed robot position
            let mut proposed_robot_move = self.robot.pos;
            match instruction {
                '^' => proposed_robot_move.y -= 1,
                '>' => proposed_robot_move.x += 1,
                'v' => proposed_robot_move.y += 1,
                '<' => proposed_robot_move.x -= 1,
                _ => ()
            }

            // might be able to collapse most of this into the rec fn
            // check for walls or boxes
            match self.locations.entry(proposed_robot_move) {
                std::collections::hash_map::Entry::Occupied(entry) => {
                    match *entry.get() {
                        // if wall { cannot move } (match arm might be redundant)
                        '#' => (),
                        // if box { check if box can move, move if yes}
                        'O' => {
                            if self.move_box(proposed_robot_move) {
                                self.robot.pos = proposed_robot_move;
                            };
                        },
                        _ => (),
                    }
                }
                std::collections::hash_map::Entry::Vacant(entry) => {
                    // if free { move robot }
                    self.robot.pos = proposed_robot_move;
                }

            }
        }

    }

    fn move_box(&mut self, proposed_move: Point<usize>,) -> bool{

        // exit conditions?

        // check for walls or boxes
        // if free { move box }
        // if wall { cannot move}
        // if box { rec check if box can move, move if yes }

        false
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
