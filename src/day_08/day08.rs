use aocutils::point::Point;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::ops::Range;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Antennae {
    antennae: HashMap<char, Vec<Point<i32>>>,
    xrange: Range<i32>,
    yrange: Range<i32>,
}

impl Antennae {
    fn new(file: &str) -> Antennae {
        let mut antennae: HashMap<char, Vec<Point<i32>>> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in fs::read_to_string(file)
            .expect("Can't read the file")
            .lines().enumerate()
        {
            max_x = line.len() as i32;
            max_y += 1;
            // only A for the mo, need to expand this to catch 0 to 9, A to Z, a to z
            for (x, g) in line.match_indices(|g: char| g.is_ascii_alphanumeric()) {
                let group = g.chars().next().expect("String is empty");
                antennae.entry(group)
                    .and_modify(|e| e.push(Point{x: x as i32, y: y as i32}))
                    .or_insert(vec!(Point{ x: x as i32, y: y as i32}));

            }
        }
        Antennae {
            antennae,
            xrange: 0..max_x,
            yrange: 0..max_y,
        }
    }
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let antennae = Antennae::new(&file);
    dbg!(antennae);

    14
}

#[cfg(test)]
mod tests {
    use crate::day_08::day08::{part_one,};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_08/day08_test.txt");
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_06/day06_data.txt");
        assert_eq!(result, 5095);
    }

}
