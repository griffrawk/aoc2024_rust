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
    while xrange.contains(&(x + dx)) && yrange.contains(&(y + dy)) {
        x += dx;
        y += dy;
        harmonics.push(Point { x, y });
    }
    let mut x = node_b.x;
    let mut y = node_b.y;
    while xrange.contains(&(x - dx)) && yrange.contains(&(y - dy)) {
        x -= dx;
        y -= dy;
        harmonics.push(Point { x, y });
    }
    harmonics
}

#[derive(Debug)]
struct AntinodeGen {
    antinode: Point<i32>,
    dx: i32,
    dy: i32,
    up: bool,
}

impl AntinodeGen {
    pub fn new(node_a: Point<i32>, node_b: Point<i32>, up: bool) -> AntinodeGen {
        let dx = node_a.x - node_b.x;
        let dy = node_a.y - node_b.y;
        if up {
            // which start point
            let antinode = node_a;
            AntinodeGen {
                antinode,
                dx,
                dy,
                up,
            }
        } else {
            let antinode = node_b;
            AntinodeGen {
                antinode,
                dx,
                dy,
                up,
            }
        }
    }
}

impl Iterator for AntinodeGen {
    type Item = Point<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.up {
            self.antinode.x += self.dx;
            self.antinode.y += self.dy;
        } else {
            self.antinode.x -= self.dx;
            self.antinode.y -= self.dy;
        }
        Some(self.antinode)
    }
}

#[allow(dead_code)]
pub fn part_one_two(file: &str) -> (usize, usize) {
    let city = City::new(&file);
    // HashSets to provide unique lists
    let mut antinodes: HashSet<Point<i32>> = HashSet::new();
    let mut harmonics: HashSet<Point<i32>> = HashSet::new();

    for group in city.antennae.values() {
        for (pos, node_a) in group.iter().enumerate().take(group.len() - 1) {
            // node_a also a harmonic antinode
            harmonics.insert(node_a.clone());
            for node_b in group[(pos + 1)..].iter() {
                // node_b also a harmonic antinode
                harmonics.insert(node_b.clone());
                let mut gen_loop = |mut antinode_gen: AntinodeGen| {
                    for (count, antinode) in
                        antinode_gen.into_iter().enumerate().take_while(|(_, a)| {
                            city.xrange.contains(&a.x) && city.yrange.contains(&a.y)
                        })
                    {
                        if count == 0 {
                            antinodes.insert(antinode.clone());
                        }
                        harmonics.insert(antinode.clone());
                    }
                };
                // Do the 'up' antinodes
                let antinode_gen = AntinodeGen::new(node_a.clone(), node_b.clone(), true);
                gen_loop(antinode_gen);
                // Do the 'down' antinodes
                let antinode_gen = AntinodeGen::new(node_a.clone(), node_b.clone(), false);
                gen_loop(antinode_gen);
            }
        }
    }
    (antinodes.len(), harmonics.len())
}

#[cfg(test)]
mod tests {
    use crate::day_08::day08::{calc_antinodes, part_one_two, AntinodeGen};
    use aocutils::point::Point;

    #[test]
    fn test_antinodes() {
        let a: Point<i32> = Point { x: 8, y: 1 };
        let b: Point<i32> = Point { x: 5, y: 2 };
        dbg!(&calc_antinodes(&a, &b));
    }

    #[test]
    fn test_antinode_gen() {
        let mut antinodes = AntinodeGen::new(Point { x: 8, y: 1 }, Point { x: 5, y: 2 }, true);
        dbg!(antinodes.next());
        dbg!(antinodes.next());
        dbg!(antinodes.next());

        let mut antinodes = AntinodeGen::new(Point { x: 8, y: 1 }, Point { x: 5, y: 2 }, false);
        dbg!(antinodes.next());
        dbg!(antinodes.next());
        dbg!(antinodes.next());
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
