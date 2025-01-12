use aocutils::point::Point;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;
use itertools::Itertools;

#[derive(Debug, Clone, Eq, PartialEq)]
struct City {
    antennae: HashMap<char, Vec<Point<i32>>>,
    xrange: Range<i32>,
    yrange: Range<i32>,
}

impl City {
    fn new(file: &str) -> City {
        let mut antennae: HashMap<char, Vec<Point<i32>>> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
            .enumerate()
        {
            max_x = line.len() as i32;
            max_y += 1;
            for (x, g) in line.match_indices(|g: char| g.is_ascii_alphanumeric()) {
                let group = g.chars().next().expect("String is empty");
                antennae
                    .entry(group)
                    .and_modify(|e| {
                        e.push(Point {
                            x: x as i32,
                            y: y as i32,
                        })
                    })
                    .or_insert(vec![Point {
                        x: x as i32,
                        y: y as i32,
                    }]);
            }
        }
        City {
            antennae,
            xrange: 0..max_x,
            yrange: 0..max_y,
        }
    }
}

fn calc_antinodes(node_a: &Point<i32>, node_b: &Point<i32>, harmonics: bool) -> Vec<Point<i32>> {
    let dx = node_a.x - node_b.x;
    let dy = node_a.y - node_b.y;
    vec!(Point { x: node_a.x + dx, y: node_a.y + dy }, Point { x: node_b.x - dx, y: node_b.y - dy })
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let city = City::new(&file);
    let mut antinodes: HashSet<Point<i32>> = HashSet::new();

    for (_, group) in city.antennae {
        for (pos, node_a) in group[0..group.len()-1].iter().enumerate() {
            for node_b in group[(pos + 1)..].into_iter() {
                for antinode in calc_antinodes(node_a, node_b) {
                    if city.xrange.contains(&antinode.x) && city.yrange.contains(&antinode.y) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use crate::day_08::day08::{part_one, calc_antinodes};
    use aocutils::point::Point;

    #[test]
    fn test_antinodes() {
        let a: Point<i32> = Point { x: 8, y: 1};
        let b: Point<i32> = Point { x: 5, y: 2};
        dbg!(&calc_antinodes(&a, &b));
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_08/day08_test.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_08/day08_data.txt");
        assert_eq!(result, 369);
    }
}
