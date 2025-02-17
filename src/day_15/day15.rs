use std::collections::HashMap;
use std::fs;
use plotters::coord::types::RangedCoordi32;
use aocutils::point::Point;
use plotters::prelude::*;

const OUTPUT_FILENAME: &str = "src/day_15/output/day_15_gen_";

#[derive(Debug, Clone, Default)]
pub struct Robot {
    pos: Point<usize>,
}

#[derive(Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
pub enum Obstacle {
    Wall,
    Box,
}

#[derive(Debug, Clone)]
pub struct Warehouse {
    robot: Robot,
    locations: HashMap<Point<usize>, Obstacle>,
    instructions: Vec<Direction>,
    plot_sequence: usize,
}

impl Warehouse {
    pub fn new(file: &str) -> Self {
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
        Warehouse { robot, locations, instructions, plot_sequence: 0 }
    }

    pub fn visual_plot(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let out = format!("{}{:06}{}", OUTPUT_FILENAME, self.plot_sequence, ".png");
        let root_area = BitMapBackend::new(&out , (1024, 1024))
            .into_drawing_area();

        root_area.fill(&WHITE).unwrap();
        let root_area = root_area.apply_coord_spec(Cartesian2d::<RangedCoordi32, RangedCoordi32>::new(
            0..50,
            0..50,
            (0..1024, 0..1024),
        ));

        let wall_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0,0),(18,18)], ShapeStyle::from(&RED).filled())
        };
        let box_block = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Rectangle::new([(0,0),(18,18)], ShapeStyle::from(&GREEN).filled())
        };
        let robot = |x: i32, y: i32| {
            return EmptyElement::at((x, y))
                + Circle::new((9,9), 9, ShapeStyle::from(&BLUE).filled())
        };

        for (pos, obstacle) in &self.locations {
            match obstacle {
                Obstacle::Wall => root_area.draw(&wall_block(pos.x as i32, pos.y as i32))?,
                Obstacle::Box => root_area.draw(&box_block(pos.x as i32, pos.y as i32))?,
            }
        }
        root_area.draw(&robot(self.robot.pos.x as i32, self.robot.pos.y as i32))?;

        root_area.present()?;
        self.plot_sequence += 1;

        Ok(())
    }

    pub fn move_robot(&mut self) {
        // todo visualise frame 0
        self.visual_plot().expect("TODO: panic message");
        for instruction in self.instructions.clone() {
            self.visual_plot().expect("TODO: panic message");
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

    pub fn move_obstacle(&mut self, proposed_move: Point<usize>, instruction: Direction) -> bool{
        match self.locations.get(&proposed_move) {
            Some(obstacle) => {
                match obstacle {
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
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use crate::day_15::day15::{Obstacle, Warehouse};

    fn part_one(file: &str) -> usize {
        let path = env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
        let mut warehouse = Warehouse::new(file);
        warehouse.move_robot();
        println!("Robot moves = {}", warehouse.plot_sequence);
        let mut res = 0;
        for (pos, c) in warehouse.locations {
            if let Obstacle::Box = c {
                res += pos.y * 100 + pos.x;
            }
        }
        res
    }

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



