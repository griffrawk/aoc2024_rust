use std::collections::HashMap;
use std::fs;
use aocutils::point::Point;

#[derive(Debug, Clone, Default)]
struct Robot {
    pos: Point<usize>,
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
enum Obstacle {
    Wall,
    Box,
}

#[derive(Debug, Clone)]
struct Warehouse {
    robot: Robot,
    locations: HashMap<Point<usize>, Obstacle>,
    instructions: Vec<Direction>,
}

impl Warehouse {
    fn new(file: &str) -> Self {
        let mut robot: Robot = Default::default();
        let mut locations: HashMap<Point<usize>, Obstacle> = HashMap::new();
        let mut instructions = Vec::new();
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
                        'O' => {
                            locations.entry(Point { x, y })
                                .or_insert(Obstacle::Box);
                        },
                        '#' => {
                            locations.entry(Point { x, y })
                                .or_insert(Obstacle::Wall);
                        },
                        '@' => {
                            robot.pos.x = x;
                            robot.pos.y = y;
                        },
                        _ => (),
                    }
                }
            } else {
                for i in line.chars() {
                    match i {
                        '^' => instructions.push(Direction::North),
                        '>' => instructions.push(Direction::East),
                        'v' => instructions.push(Direction::South),
                        '<' => instructions.push(Direction::West),
                        _ => (),
                    }
                }
            }
        }
        Warehouse { robot, locations, instructions }
    }

    fn move_robot(&mut self) {
        for instruction in self.instructions.clone() {
            // calc proposed robot position
            let mut proposed_robot_move = self.robot.pos;
            // sort out an enum for instruction...
            match instruction {
                Direction::North => proposed_robot_move.y -= 1,
                Direction::East => proposed_robot_move.x += 1,
                Direction::South => proposed_robot_move.y += 1,
                Direction::West => proposed_robot_move.x -= 1,
            }

            if self.move_obstacle(proposed_robot_move, instruction) {
                self.robot.pos = proposed_robot_move;
            }
        }
    }

    fn move_obstacle(&mut self, proposed_move: Point<usize>, instruction: Direction) -> bool{
        let loc = self.locations.get(&proposed_move);
        match loc {
            Some(e) => {
                match e {
                    // if wall then cannot move
                    Obstacle::Wall => false,
                    // if box { check if box can move, move if yes}
                    Obstacle::Box => {
                        let mut next_move = proposed_move;
                        match instruction {
                            Direction::North => next_move.y -= 1,
                            Direction::East => next_move.x += 1,
                            Direction::South => next_move.y += 1,
                            Direction::West => next_move.x -= 1,
                        }
                        return if self.move_obstacle(next_move, instruction) {
                            // insert box at next move
                            self.locations.entry(next_move).or_insert(Obstacle::Box);
                            // remove the box at proposed_move
                            self.locations.remove(&proposed_move);
                            true
                        } else {
                            false
                        }
                    },
                }
            },
            None => {
                return true;
            }
        }
    }
}

fn part_one(file: &str) -> usize {
    let mut warehouse = Warehouse::new(file);
    warehouse.move_robot();
    let mut res = 0;
    for (pos, c) in warehouse.locations {
        if let Obstacle::Box = c {
            res += pos.y * 100 + pos.x;
        }
    }
    res
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
        assert_eq!(result, 1421727);
    }

}
