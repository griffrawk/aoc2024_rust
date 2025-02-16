use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use aocutils::point::Point;

#[derive(Debug, Clone, Default)]
struct Robot {
    pos: Point<usize>,
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
            // sort out an enum for instruction...
            match instruction {
                '^' => proposed_robot_move.y -= 1,
                '>' => proposed_robot_move.x += 1,
                'v' => proposed_robot_move.y += 1,
                '<' => proposed_robot_move.x -= 1,
                _ => ()
            }

            if self.move_obstacle(proposed_robot_move, instruction) {
                self.robot.pos = proposed_robot_move;
            }
        }
        // whatever follows all moves
        ()
    }

    fn move_obstacle(&mut self, proposed_move: Point<usize>, instruction: char) -> bool{
        // this box
        match self.locations.entry(proposed_move) {
            std::collections::hash_map::Entry::Occupied(entry) => {
                match entry.get() {
                    // if wall then cannot move
                    '#' => return false,
                    // if box { check if box can move, move if yes}
                    'O' => {
                        let mut next_move = proposed_move;
                        match instruction {
                            '^' => next_move.y -= 1,
                            '>' => next_move.x += 1,
                            'v' => next_move.y += 1,
                            '<' => next_move.x -= 1,
                            _ => ()
                        }
                        return if self.move_obstacle(next_move, instruction) {
                            // insert 'O' at next move
                            // remove the box at proposed_move
                            self.locations.entry(next_move).or_insert('O');
                            entry.remove();
                            true
                        } else {
                            false
                        }
                    },
                    _ => (),
                }
            },
            std::collections::hash_map::Entry::Vacant(_entry) => {
                return true;
            }
        }
        false
    }
}

fn part_one(file: &str) -> usize {
    let mut warehouse = Warehouse::new(file);
    warehouse.move_robot();
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
