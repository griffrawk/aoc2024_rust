use std::collections::HashMap;
use std::fs;

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
    direction: Direction,
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let contents = fs::read_to_string(file).expect("Can't read the file");
    let mut obstacles: HashMap<Point, usize> = HashMap::new();
    let mut visited: HashMap<Point, usize> = HashMap::new();
    let mut guard = Guard { pos: Point { x: 0, y: 0} , direction: Direction::North };

    let mut max_x = 0;
    let mut max_y = 0;
    for line in contents.lines() {
        max_x = line.len();
        for (x , _) in line.match_indices("#") {
            obstacles.insert(Point {x: x as i32, y: max_y }, 0);
        }
        for (g, _) in line.match_indices("^") {
            // should only be one, but if not, it uses the last one found
            guard.pos.x = g as i32;
            guard.pos.y = max_y;
        }
        max_y += 1;
    }

    let xrange = 0..max_x as i32;
    let yrange= 0..max_y as i32;

    // while guard still on grid
    while xrange.contains(&guard.pos.x) && yrange.contains(&guard.pos.y) {
        // record guard position as visited
        visited.insert(Point { x: guard.pos.x, y: guard.pos.y }, 0);
        loop {
            // until unblocked move found
            let mut poss = Point { x: 0, y: 0 };
            let poss_direction: Direction;
            match guard.direction {
                Direction::North => {
                    poss.x = guard.pos.x;
                    poss.y = guard.pos.y - 1;
                    poss_direction = Direction::East;
                }
                Direction::East => {
                    poss.x = guard.pos.x + 1;
                    poss.y = guard.pos.y;
                    poss_direction = Direction::South;
                }
                Direction::South => {
                    poss.x = guard.pos.x;
                    poss.y = guard.pos.y + 1;
                    poss_direction = Direction::West;
                }
                Direction::West => {
                    poss.x = guard.pos.x - 1;
                    poss.y = guard.pos.y;
                    poss_direction = Direction::North;
                }
            }
            if !obstacles.contains_key(&poss) {
                // valid move
                guard.pos.x = poss.x;
                guard.pos.y = poss.y;
                break
            } else {
                // change direction and try again
                guard.direction = poss_direction;
            }
        }
    }
    // return visited count
    visited.len()
}

#[cfg(test)]
mod tests {
    use crate::day_06::day06::{part_one};

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
