#[cfg(test)]
mod tests {
    use aocutils::point::Point;
    use rayon::prelude::*;
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::ops::Range;

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Debug, Clone)]
    struct Guard {
        position: Point<i32>,
        xrange: Range<i32>,
        yrange: Range<i32>,
        direction: Direction,
    }

    impl Guard {
        fn new(file: &str) -> Guard {
            let mut position = Point { x: 0, y: 0 };
            let direction = Direction::North;
            let mut max_y = 0;
            let mut max_x = 0;
            for line in fs::read_to_string(file)
                .expect("Can't read the file")
                .lines()
            {
                max_x = line.len() as i32;
                // should only be one, but if not, it uses the last one found
                if let Some((g, _)) = line.match_indices("^").last() {
                    position = Point {
                        x: g as i32,
                        y: max_y,
                    };
                }
                max_y += 1;
            }

            Guard {
                position,
                xrange: 0..max_x,
                yrange: 0..max_y,
                direction,
            }
        }

        fn reset(&mut self, new_obs: Point<i32>, direction: Direction) {
            self.position = new_obs;
            match direction {
                Direction::North => self.position.y += 1,
                Direction::East => self.position.x -= 1,
                Direction::South => self.position.y -= 1,
                Direction::West => self.position.x += 1,
            }
            self.direction = direction;
        }

        fn step(&mut self, obstacles: &Obstacles) {
            let mut safety_net = 0;
            // if guard turns right 4 times, it is trapped, so safety net
            while safety_net < 3 {
                // until unblocked move found or safety net
                let mut possible = self.position.clone();
                let poss_direction = match self.direction {
                    Direction::North => {
                        possible.y -= 1;
                        Direction::East
                    }
                    Direction::East => {
                        possible.x += 1;
                        Direction::South
                    }
                    Direction::South => {
                        possible.y += 1;
                        Direction::West
                    }
                    Direction::West => {
                        possible.x -= 1;
                        Direction::North
                    }
                };
                if !obstacles.obstacles.contains(&possible) {
                    // valid move
                    self.position = possible;
                    break;
                } else {
                    // change direction and try again
                    self.direction = poss_direction;
                }
                safety_net += 1;
            }
        }

        fn walk(&mut self, obstacles: &Obstacles) -> (HashMap<Point<i32>, Direction>, bool) {
            let mut visited: HashMap<Point<i32>, Direction> = HashMap::new();
            let mut stuck = false;
            // while guard still on grid
            while self.xrange.contains(&self.position.x) && self.yrange.contains(&self.position.y) {
                // check if guard has been here before in same direction, stuck if so...
                match visited.entry(self.position.clone()) {
                    std::collections::hash_map::Entry::Occupied(entry) => {
                        if *entry.get() == self.direction {
                            stuck = true;
                            break;
                        }
                    }
                    std::collections::hash_map::Entry::Vacant(entry) => {
                        entry.insert(self.direction);
                    }
                }
                self.step(obstacles);
            }
            (visited, stuck)
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Obstacles {
        obstacles: HashSet<Point<i32>>,
    }

    impl Obstacles {
        fn new(file: &str) -> Obstacles {
            let mut obstacles: HashSet<Point<i32>> = HashSet::new();
            for (y, line) in fs::read_to_string(file)
                .expect("Can't read the file")
                .lines()
                .enumerate()
            {
                for (x, _) in line.match_indices("#") {
                    obstacles.insert(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
            Obstacles { obstacles }
        }
    }

    fn part_one(file: &str) -> usize {
        let obstacles = Obstacles::new(&file);
        let mut lab_guard = Guard::new(&file);

        // return length of visited points
        lab_guard.walk(&obstacles).0.len()
    }

    fn part_two(file: &str) -> usize {
        let mut obstacles = Obstacles::new(&file);
        let mut lab_guard = Guard::new(&file);

        // check if a new obs at any of the initial visited points would cause a loop
        lab_guard
            .walk(&obstacles)
            .0
            .into_iter()
            .map(|(new_obs, direction)| {
                // add new Point clone obstacle to a clone of obstacles
                obstacles.obstacles.insert(new_obs.clone());
                // reset the guard to the last position before this new obstacle, opposite to direction.
                lab_guard.reset(new_obs.clone(), direction);
                // then run the guard thru new obstacle course
                let (_, stuck) = lab_guard.walk(&obstacles);
                // remove obstacle
                obstacles.obstacles.remove(&new_obs.clone());
                if stuck {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    fn part_two_parallel(file: &str) -> usize {
        let obstacles = Obstacles::new(&file);
        let mut lab_guard = Guard::new(&file);

        // check if a new obs at any of the initial visited points would cause a loop
        lab_guard
            .walk(&obstacles)
            .0
            .into_par_iter()
            .map(|(new_obs, direction)| {
                // per thread clones
                // add new Point clone obstacle to a clone of obstacles
                let mut clone_obstacles = obstacles.clone();
                clone_obstacles.obstacles.insert(new_obs.clone());
                // reset the guard to the last position before this new obstacle, opposite to direction.
                let mut clone_guard = lab_guard.clone();
                clone_guard.reset(new_obs.clone(), direction);
                // then run the guard thru new obstacle course
                let (_, stuck) = clone_guard.walk(&clone_obstacles);
                // no need to remove the obstacle as we are parallel running
                if stuck {
                    1
                } else {
                    0
                }
            })
            .sum()
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/bin/day06/day06_test.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/bin/day06/day06_data.txt");
        assert_eq!(result, 5095);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/bin/day06/day06_test.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two_test_parallel() {
        let result = part_two_parallel("src/bin/day06/day06_test.txt");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/bin/day06/day06_data.txt");
        assert_eq!(result, 1933);
    }

    #[test]
    fn test_part_two_data_parallel() {
        let result = part_two_parallel("src/bin/day06/day06_data.txt");
        assert_eq!(result, 1933);
    }
}

fn main() {}
