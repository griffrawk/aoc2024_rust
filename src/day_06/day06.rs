use std::collections::HashMap;
use std::fs;
use std::ops::Range;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
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

    fn move_guard(&mut self, obstacles: &Obstacles) {
        loop {
            // until unblocked move found
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
            if !obstacles.obstacles.contains_key(&poss) {
                // valid move
                self.pos = poss;
                break;
            } else {
                // change direction and try again
                self.direction = poss_direction;
            }
        }
    }

    fn walk_guard(&mut self, obstacles: &Obstacles) -> HashMap<Point, usize> {
        let mut visited: HashMap<Point, usize> = HashMap::new();
        // while guard still on grid
        while self.xrange.contains(&self.pos.x) && self.yrange.contains(&self.pos.y) {
            // record guard position as visited
            visited.insert(
                Point {
                    x: self.pos.x,
                    y: self.pos.y,
                },
                0,
            );
            self.move_guard(&obstacles);
        }
        visited
    }
}

struct Obstacles {
    obstacles: HashMap<Point, usize>,
}

impl Obstacles {
    fn new(file: &str) -> Obstacles {
        let mut obstacles: HashMap<Point, usize> = HashMap::new();
        let mut y = 0;
        for line in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
        {
            for (x, _) in line.match_indices("#") {
                obstacles.insert(Point { x: x as i32, y: y }, 0);
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
    guard.walk_guard(&obstacles).len()
}

#[cfg(test)]
mod tests {
    use crate::day_06::day06::part_one;

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
}
