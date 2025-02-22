#[cfg(test)]
mod tests {

    use aocutils::point::Point;
    use std::collections::HashMap;
    use std::{env, fs};
    use plotters::coord::types::RangedCoordi32;
    use plotters::prelude::*;
    
    const OUTPUT_FILENAME: &str = "src/bin/day15/output_part2_data/day15_gen";

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
        locations_rollback: HashMap<Point<usize>, Obstacle>,
        instructions: Vec<Direction>,
        plot_sequence: usize,
    }

    impl Warehouse {
        fn new(file: &str) -> Self {
            let mut robot: Robot = Default::default();
            let mut locations: HashMap<Point<usize>, Obstacle> = HashMap::new();
            let locations_rollback: HashMap<Point<usize>, Obstacle> = HashMap::new();
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
                        let x2 = x * 2;
                        match c {
                            'O' => {
                                locations
                                    .entry(Point { x: x2, y })
                                    .or_insert(Obstacle::Box);
                            }
                            '#' => {
                                locations
                                    .entry(Point { x: x2, y })
                                    .or_insert(Obstacle::Wall);
                            }
                            '@' => {
                                robot.pos.x = x2;
                                robot.pos.y = y;
                            }
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
            Warehouse {
                robot,
                locations,
                locations_rollback,
                instructions,
                plot_sequence: 0,
            }
        }

        fn visual_plot(&mut self, instruction: &Direction) -> Result<(), Box<dyn std::error::Error>> {
            let out = format!("{}_{:06}_{}{}", 
                                OUTPUT_FILENAME,
                                self.plot_sequence, 
                                match instruction {
                                    Direction::North => '^',
                                    Direction::East => '>',
                                    Direction::South => 'v',
                                    Direction::West => '<',
                                },
                                ".png");
            let root_area = BitMapBackend::new(&out, (1024, 1024)).into_drawing_area();

            root_area.fill(&WHITE).unwrap();
            let root_area = root_area.apply_coord_spec(
                Cartesian2d::<RangedCoordi32, RangedCoordi32>::new(0..100, 0..100, (0..1024, 0..1024)),
            );

            let wall_block = |x: i32, y: i32| {
                return EmptyElement::at((x, y))
                    + Rectangle::new([(0, 0), (18, 9)], ShapeStyle::from(&RED).filled());
            };
            let box_block = |x: i32, y: i32| {
                return EmptyElement::at((x, y))
                    + Rectangle::new([(0, 0), (18, 9)], ShapeStyle::from(&GREEN).filled());
            };
            let robot = |x: i32, y: i32| {
                return EmptyElement::at((x, y))
                    + Circle::new((4, 4), 4, ShapeStyle::from(&BLUE).filled());
            };

            for (pos, obstacle) in &self.locations {
                match obstacle {
                    Obstacle::Wall => root_area.draw(&wall_block(pos.x as i32, pos.y as i32))?,
                    Obstacle::Box => root_area.draw(&box_block(pos.x as i32, pos.y as i32))?,
                }
            }
            root_area.draw(&robot(self.robot.pos.x as i32, self.robot.pos.y as i32))?;
            root_area.present()?;
            Ok(())
        }

        fn move_robot(&mut self) {
            for instruction in self.instructions.clone() {
                // Make a list of moves for the robot to check if there's an obstacle
                let mut move_list: Vec<Point<usize>> = Vec::new();
                let mut proposed_robot_move = self.robot.pos;
                let mut obstacle_check = self.robot.pos;
                // There's more to check when moving north or south, push 2 checks,
                // otherwise 1 check each for East, West
                match instruction {
                    Direction::North => {
                        proposed_robot_move.y -= 1;
                        
                        //    []
                        // ^  @     x + 0, y + -1
                        obstacle_check.y = self.robot.pos.y - 1;
                        move_list.push(obstacle_check);
                        
                        //   []
                        // ^  @     x + -1, y + -1
                        obstacle_check.x = self.robot.pos.x - 1;
                        move_list.push(obstacle_check);
                    },
                    Direction::East => {
                        proposed_robot_move.x += 1;
                        
                        // > @[]    x + 1, y + 0
                        obstacle_check.x = self.robot.pos.x + 1;
                        move_list.push(obstacle_check);
                    },
                    Direction::South => {
                        proposed_robot_move.y += 1;
                        
                        // v  @
                        //    []    x + 0, y + 1
                        obstacle_check.y = self.robot.pos.y + 1;
                        move_list.push(obstacle_check);
                        
                        // v  @
                        //   []     x + -1, y + 1
                        obstacle_check.x = self.robot.pos.x - 1;
                        move_list.push(obstacle_check);
                    },
                    Direction::West => {
                        proposed_robot_move.x -= 1;
                        
                        // < []@    x + -2, y + 0
                        obstacle_check.x = self.robot.pos.x - 2;
                        move_list.push(obstacle_check);
                    },
                }
                // Process each move, only move robot if all moves true
                self.locations_rollback = self.locations.clone();
                let res: Vec<bool> = move_list
                    .iter()
                    .map(|prop| self.move_obstacle(*prop, instruction.clone()))
                    .collect();
                if !res.contains(&false) {
                    self.robot.pos = proposed_robot_move;
                    self.locations = self.locations_rollback.clone();
                }
                // self.visual_plot(&instruction).expect("TODO: panic message");
                self.plot_sequence += 1;
            }
        }

        fn move_obstacle(&mut self, proposed_move: Point<usize>, instruction: Direction) -> bool {
            match self.locations_rollback.get(&proposed_move) {
                Some(obstacle) => {
                    match obstacle {
                        // wall blocks movement
                        Obstacle::Wall => false,
                        // if box { check if box can move, move if yes}
                        Obstacle::Box => {
                            // Make a list of moves for a box to check for an obstacle
                            let mut move_list: Vec<Point<usize>> = Vec::new();
                            let mut next_move = proposed_move;
                            let mut obstacle_check = proposed_move;
                            // There's more to check when moving north or south, push 3 checks,
                            // otherwise 1 check each for East, West
                            match instruction {
                                Direction::North => {
                                    next_move.y -= 1;

                                    //   []
                                    // ^  []     x + -1, y + -1
                                    obstacle_check.x = proposed_move.x - 1;
                                    obstacle_check.y = proposed_move.y - 1;
                                    move_list.push(obstacle_check);

                                    //   []
                                    // ^ []     x + 0, y + -1
                                    obstacle_check.x = proposed_move.x;
                                    move_list.push(obstacle_check);
                                    
                                    //    []
                                    // ^ []     x + 1, y + -1
                                    obstacle_check.x = proposed_move.x + 1;
                                    move_list.push(obstacle_check);
                                },
                                Direction::East => {
                                    next_move.x += 1;

                                    // > [][]    x + 2, y + 0
                                    obstacle_check.x = proposed_move.x + 2;
                                    obstacle_check.y = proposed_move.y;
                                    move_list.push(obstacle_check);
                                },
                                Direction::South => {
                                    next_move.y += 1;

                                    // v  []
                                    //   []    x + -1, y + 1
                                    obstacle_check.x = proposed_move.x - 1;
                                    obstacle_check.y = proposed_move.y + 1;
                                    move_list.push(obstacle_check);

                                    // v  []
                                    //    []     x + 0, y + 1
                                    obstacle_check.x = proposed_move.x;
                                    move_list.push(obstacle_check);
                                    
                                    // v  []
                                    //     []     x + 1, y + 1
                                    obstacle_check.x = proposed_move.x + 1;
                                    move_list.push(obstacle_check);
                                },
                                Direction::West => {
                                    next_move.x -= 1;

                                    // < [][]    x - 2, y + 0
                                    obstacle_check.x = proposed_move.x - 2;
                                    obstacle_check.y = proposed_move.y;
                                    move_list.push(obstacle_check);
                                },
                            }
                            // Process each move, only move box if all true
                            let res: Vec<bool> = move_list
                                .iter()
                                .map(|prop| self.move_obstacle(*prop, instruction.clone()))
                                .collect();
                            if !res.contains(&false) {
                                // insert box at next_move
                                self.locations_rollback.entry(next_move).or_insert(Obstacle::Box);
                                // remove the box at proposed_move
                                self.locations_rollback.remove(&proposed_move);
                                true
                            } else {
                                false
                            }
                        }
                    }
                }
                None => true,
            }
        }
    }

    fn part_two(file: &str) -> usize {
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
    #[ignore]
    fn test_part_two_basic() {
        let result = part_two("src/bin/day15/day15_basic.txt");
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/bin/day15/day15_test.txt");
        assert_eq!(result, 9021);
    }

    #[test]
    // #[ignore = "Best run with --profile release, takes a long time"]
    fn test_part_two_data() {
        let result = part_two("src/bin/day15/day15_data.txt");
        assert_eq!(result, 1463160);
    }
}
