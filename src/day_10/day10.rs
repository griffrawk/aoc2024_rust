use aocutils::point::Point;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

// Part 1 structs & impl
#[derive(Debug)]
struct TopoMap {
    heights: HashMap<Point<i32>, u32>,
    trailheads: Vec<Point<i32>>,
    visited: HashSet<Point<i32>>,
    xrange: Range<i32>,
    yrange: Range<i32>,
    res: usize,
}

impl TopoMap {
    fn new(file: &str) -> TopoMap {
        let mut heights = HashMap::new();
        let mut trailheads = Vec::new();
        let mut xrange = 0..1;
        let mut yrange = 0..1;
        for (y, line) in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
            .enumerate()
        {
            xrange = 0..line.len() as i32;
            yrange = 0..y as i32 + 1;
            for (x, height) in line.chars().enumerate() {
                let height = height.to_digit(10).unwrap_or_default();
                heights.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    height,
                );
                if height == 0 {
                    trailheads.push(Point {x: x as i32, y: y as i32});
                }
            }
        }
        let res = 0;
        let visited = HashSet::new();

        TopoMap {
            heights,
            trailheads,
            visited,
            xrange,
            yrange,
            res,
        }
    }

    fn walk_trails(&mut self, part_two: bool) -> usize {
        for head in self.trailheads.clone() {
            self.visited.drain();
            self.walk(head, part_two);
        }
        self.res
    }

    fn walk(&mut self, pos: Point<i32>, part_two: bool) {
        let height = self.heights[&pos];
        // For part 2, ignore memos, so we find all routes even if partially duplicate
        if !part_two {
            if self.visited.contains(&pos) { return }
            self.visited.insert(pos);
        }
        if height == 9 {
            self.res += 1;
            return;
        }
        // look for next higher
        let next_height = height + 1;
        // check N, E, S, W
        for next_pos in pos.cardinal_points() {
            if self.xrange.contains(&next_pos.x) && self.yrange.contains(&next_pos.y) {
                if self.heights[&next_pos] == next_height {
                    self.walk(next_pos, part_two)
                }
            }
        }
    }
}

// mains
#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    TopoMap::new(&file).walk_trails(false)
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> usize {
    TopoMap::new(&file).walk_trails(true)
}

#[cfg(test)]
mod tests {
    use crate::day_10::day10::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_10/day10_test.txt");
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_10/day10_data.txt");
        assert_eq!(result, 430);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_10/day10_test.txt");
        assert_eq!(result, 81);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_10/day10_data.txt");
        assert_eq!(result, 928);
    }
}
