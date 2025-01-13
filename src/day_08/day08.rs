use aocutils::point::Point;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

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

fn calc_antinodes(node_a: &Point<i32>, node_b: &Point<i32>) -> Vec<Point<i32>> {
    let dx = node_a.x - node_b.x;
    let dy = node_a.y - node_b.y;
    vec![
        Point {
            x: node_a.x + dx,
            y: node_a.y + dy,
        },
        Point {
            x: node_b.x - dx,
            y: node_b.y - dy,
        },
    ]
}

fn calc_harmonics(
    node_a: &Point<i32>,
    node_b: &Point<i32>,
    xrange: &Range<i32>,
    yrange: &Range<i32>,
) -> Vec<Point<i32>> {
    // add harmonic antinodes to output until they go beyond ranges
    let dx = node_a.x - node_b.x;
    let dy = node_a.y - node_b.y;
    let mut harmonics: Vec<Point<i32>> = Vec::new();
    let mut x = node_a.x;
    let mut y = node_a.y;
    loop {
        x += dx;
        y += dy;
        if xrange.contains(&x) && yrange.contains(&y) {
            harmonics.push(Point { x, y });
        } else {
            break;
        }
    }
    let mut x = node_b.x;
    let mut y = node_b.y;
    loop {
        x -= dx;
        y -= dy;
        if xrange.contains(&x) && yrange.contains(&y) {
            harmonics.push(Point { x, y });
        } else {
            break;
        }
    }
    harmonics
}

#[allow(dead_code)]
pub fn part_one_two(file: &str) -> (usize, usize) {
    let city = City::new(&file);
    // HashSets to provide unique lists
    let mut antinodes: HashSet<Point<i32>> = HashSet::new();
    let mut harmonics: HashSet<Point<i32>> = HashSet::new();

    for (_, group) in city.antennae {
        for (pos, node_a) in group[0..group.len() - 1].iter().enumerate() {
            // node_a also a harmonic antinode
            harmonics.insert(node_a.clone());
            for node_b in group[(pos + 1)..].iter() {
                // node_b also a harmonic antinode
                harmonics.insert(node_b.clone());
                for antinode in calc_antinodes(node_a, node_b) {
                    if city.xrange.contains(&antinode.x) && city.yrange.contains(&antinode.y) {
                        antinodes.insert(antinode);
                    }
                }
                for harmonic in calc_harmonics(node_a, node_b, &city.xrange, &city.yrange) {
                    harmonics.insert(harmonic);
                }
            }
        }
    }
    (antinodes.len(), harmonics.len())
}

#[cfg(test)]
mod tests {
    use crate::day_08::day08::{calc_antinodes, part_one_two};
    use aocutils::point::Point;

    #[test]
    fn test_antinodes() {
        let a: Point<i32> = Point { x: 8, y: 1 };
        let b: Point<i32> = Point { x: 5, y: 2 };
        dbg!(&calc_antinodes(&a, &b));
    }

    #[test]
    fn test_part_one_two_test() {
        let result = part_one_two("src/day_08/day08_test.txt");
        assert_eq!(result, (14, 34));
    }

    #[test]
    fn test_part_one_two_data() {
        let result = part_one_two("src/day_08/day08_data.txt");
        assert_eq!(result, (369, 1169));
    }
}
