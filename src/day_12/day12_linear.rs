use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use aocutils::point::Point;

#[allow(dead_code)]
fn linear_part_one(file: &str) -> usize {
    // plots addressable by Point
    let mut farm: HashMap<Point<i32>, char> = HashMap::new();
    let mut regions: HashMap<char, (usize, usize)> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in  fs::read_to_string(file)
        .expect("Can't read the file")
        .lines()
        .enumerate() {
        max_y = y as i32;
        for (x, c) in line.chars().enumerate() {
            max_x = x as i32;
            farm.insert(Point {x: x as i32, y: y as i32}, c);
        }
    }
    let xrange = 0..max_x + 1;
    let yrange = 0..max_y + 1;
    // process regions
    for (plot, crop) in &farm {
        let mut plot_perimeter = 4;
        for neigbour in plot.cardinal_points() {
            if xrange.contains(&neigbour.x) && yrange.contains(&neigbour.y) {
                if farm[&neigbour] == *crop {
                    plot_perimeter -= 1;
                }
            }
        }
        // todo ah but, is this a new clump of crop or an existing one? can't clump all
        //  plots of a crop together
        regions.entry(*crop)
            .and_modify(| c | {
                c.0 += 1;
                c.1 += plot_perimeter;
            })
            .or_insert((1, plot_perimeter));
    }
    // calculate cost,
    regions.iter().map(|(_, (area, perimeter))| area * perimeter).sum()
}


#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    linear_part_one(file)
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> usize {
    194557
}

#[cfg(test)]
mod tests {
    use crate::day_12::day12_linear::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_12/day12_test.txt");
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_12/day12_data.txt");
        assert_eq!(result, 194557);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_12/day12_test.txt");
        assert_eq!(result, 65601038650482);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_12/day12_data.txt");
        assert_eq!(result, 231532558973909);
    }
}
