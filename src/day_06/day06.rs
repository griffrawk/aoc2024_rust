use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;
use rayon::prelude::*;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone)]
struct Guard {
    pos: Point,
    xrange: Range<i32>,
    yrange: Range<i32>,
    direction: Direction,
}

impl Guard {
    fn new(file: &str) -> Guard {
        let mut pos = Point { x: 0, y: 0 };
        let direction = Direction::North;
        let mut max_y = 0;
        let mut max_x = 0;
        for line in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
        {
            max_x = line.len() as i32;
            for (g, _) in line.match_indices("^") {
                // should only be one, but if not, it uses the last one found
                pos.x = g as i32;
                pos.y = max_y;
            }
            max_y += 1;
        }
        let xrange = 0..max_x;
        let yrange = 0..max_y;

        Self {
            pos,
            xrange,
            yrange,
            direction,
        }
    }

    fn reset(&mut self, new_obs: Point, direction: Direction) {
        match direction {
            Direction::North => {
                // guard starts to the south pointing north
                self.pos.x = new_obs.x;
                self.pos.y = new_obs.y + 1;
                self.direction = Direction::North;
            }
            Direction::East => {
                // guard starts to the west pointing east
                self.pos.x = new_obs.x - 1;
                self.pos.y = new_obs.y;
                self.direction = Direction::East;
            }
            Direction::South => {
                // guard starts to the north pointing south
                self.pos.x = new_obs.x;
                self.pos.y = new_obs.y - 1;
                self.direction = Direction::South;
            }
            Direction::West => {
                // guard starts to the east pointing west
                self.pos.x = new_obs.x + 1;
                self.pos.y = new_obs.y;
                self.direction = Direction::West;
            }
        }
    }

    fn step(&mut self, obstacles: &Obstacles) {
        let mut safety_net = 0;
        while safety_net < 3 {
            // until unblocked move found or safety net
            let mut poss = Point { x: 0, y: 0 };
            let poss_direction: Direction;
            match self.direction {
                Direction::North => {
                    poss.x = self.pos.x;
                    poss.y = self.pos.y - 1;
                    poss_direction = Direction::East;
                }
                Direction::East => {
                    poss.x = self.pos.x + 1;
                    poss.y = self.pos.y;
                    poss_direction = Direction::South;
                }
                Direction::South => {
                    poss.x = self.pos.x;
                    poss.y = self.pos.y + 1;
                    poss_direction = Direction::West;
                }
                Direction::West => {
                    poss.x = self.pos.x - 1;
                    poss.y = self.pos.y;
                    poss_direction = Direction::North;
                }
            }
            if !obstacles.obstacles.contains(&poss) {
                // valid move
                self.pos = poss;
                break;
            } else {
                // change direction and try again
                self.direction = poss_direction;
            }
            safety_net += 1;
        }
    }

    fn walk(&mut self, obstacles: &Obstacles) -> (HashMap<Point, Direction>, bool) {
        let mut visited: HashMap<Point, Direction> = HashMap::new();
        let mut stuck = false;
        // while guard still on grid
        while self.xrange.contains(&self.pos.x) && self.yrange.contains(&self.pos.y) {
            // check if guard has been here before in same direction, stuck if so...
            let previous = visited.get(&Point {
                x: self.pos.x,
                y: self.pos.y,
            });
            match previous {
                Some(s) => {
                    if *s == self.direction {
                        // been here before in same direction
                        stuck = true;
                        break;
                    }
                }
                None => {
                    // record guard position as visited before move
                    visited.insert(
                        Point {
                            x: self.pos.x,
                            y: self.pos.y,
                        },
                        self.direction,
                    );
                }
            }
            self.step(&obstacles);
        }
        (visited, stuck)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Obstacles {
    obstacles: HashSet<Point>,
}

impl Obstacles {
    fn new(file: &str) -> Obstacles {
        let mut obstacles: HashSet<Point> = HashSet::new();
        let mut y = 0;
        for line in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
        {
            for (x, _) in line.match_indices("#") {
                obstacles.insert(Point { x: x as i32, y: y });
            }
            y += 1;
        }
        Self { obstacles }
    }
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let obstacles = Obstacles::new(&file);
    let mut guard = Guard::new(&file);

    // return length of visited points
    guard.walk(&obstacles).0.len()
}

pub fn part_two(file: &str) -> usize {
    let mut obstacles = Obstacles::new(&file);
    let mut guard = Guard::new(&file);

    // check if a new obs at any of the initial visited points would cause a loop
    let (visited, _) = guard.walk(&obstacles);
    visited.into_iter().map( |(new_obs, direction)| {
        // add new obstacle
        obstacles.obstacles.insert(new_obs.clone());
        // reset the guard to the last position before this new obstacle, opposite to direction.
        guard.reset(new_obs.clone(), direction);

        let (_, stuck) = guard.walk(&obstacles);
        obstacles.obstacles.remove(&new_obs.clone());
        if stuck {
            return 1;
        }
        // remove obstacle
        0
    }).sum()
}

pub fn part_two_parallel(file: &str) -> usize {
    let obstacles = Obstacles::new(&file);
    let mut guard = Guard::new(&file);

    // check if a new obs at any of the initial visited points would cause a loop
    let (visited, _) = guard.walk(&obstacles);
    visited.into_par_iter().map(|(new_obs, direction) | {
        // add new obstacle
        let mut clone_obstacles = obstacles.clone();
        let mut clone_guard = guard.clone();
        clone_obstacles.obstacles.insert(new_obs.clone());
        // reset the guard to the last position before this new obstacle, opposite to direction.
        clone_guard.reset(new_obs.clone(), direction);

        let (_, stuck) = clone_guard.walk(&clone_obstacles);
        if stuck {
            return 1;
        }
        0
    }).sum()
}

#[cfg(test)]
mod tests {
    use crate::day_06::day06::{part_one, part_two, part_two_parallel};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_06/day06_test.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_06/day06_data.txt");
        assert_eq!(result, 5095);
    }

   #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_06/day06_test.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two_test_parallel() {
        let result = part_two_parallel("src/day_06/day06_test.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_06/day06_data.txt");
        assert_eq!(result, 1933);
    }

    #[test]
    fn test_part_two_data_parallel() {
        let result = part_two_parallel("src/day_06/day06_data.txt");
        assert_eq!(result, 1933);
    }
}
