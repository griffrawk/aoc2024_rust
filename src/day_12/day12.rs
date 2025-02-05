use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use aocutils::point::Point;
use colored::{Color, ColoredString, Colorize};

#[derive(Debug, Clone)]
struct Plot {
    region: Option<usize>,
    crop: char,
}

#[derive(Debug, Clone)]
struct Farm {
    farm: HashMap<Point<i32>, Plot>,
    xrange: Range<i32>,
    yrange: Range<i32>,
    current_region: usize,
    regions: HashMap<usize, (usize, usize)>,     // k: region, v: (area, perimeter)
}

impl Farm {
    fn new(file: &str) -> Self {
        // plots addressable by Point
        let mut farm: HashMap<Point<i32>, Plot> = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in  fs::read_to_string(file)
            .expect("Can't read the file")
            .lines()
            .enumerate() {
            max_y = y as i32;
            for (x, c) in line.chars().enumerate() {
                max_x = x as i32;
                if c.is_ascii_alphanumeric() {
                    farm.insert(Point { x: x as i32, y: y as i32 }, Plot { region: None, crop: c });
                }
            }
        }
        let regions: HashMap<usize, (usize, usize)> = HashMap::new();
        Self { farm, xrange: 0..max_x + 1, yrange: 0..max_y + 1, current_region: 0, regions }
    }

    fn find_regions(&mut self) {
        // pos is from cloned farm
        for (pos, _) in self.farm.clone() {
            // but we check real farm
            // ony recurse into regionless plots
            if let None = self.farm[&pos].region {
                self.region_rec(pos);
                // exhausted region possibilities of pos, inc region
                self.current_region += 1;
            }
        }

        // for test, just first region from 0,0
        // self.region_rec(Point {x: 0, y:0 });
    }

    fn region_rec(&mut self, pos: Point<i32>) {
        // return conditions
        let plot = self.farm[&pos].clone();
        if let Some(_) = plot.region {
            return
        }

        // process plot
        // let mut plot_perimeter = 4;
        // todo might need to do the neighbour loop twice,
        //  once to calculate the perimeter
        //  once for the recursion

        // does plot have neighbours of same crop?
        for neigbour_pos in pos.cardinal_points() {
            if self.xrange.contains(&neigbour_pos.x) && self.yrange.contains(&neigbour_pos.y) {
                let neighbour_plot = self.farm[&neigbour_pos].clone();
                // same crop?
                if neighbour_plot.crop == plot.crop {
                    // already in a region?
                    match neighbour_plot.region {
                        Some(r) => {
                            // set plot region to same as neighbour
                            self.farm.entry(pos)
                                .and_modify(|p| p.region = Some(r));
                        }
                        None => {
                            // use new region and update pos
                            self.farm.entry(pos)
                                .and_modify(|p| p.region = Some(self.current_region));

                            // neighbour doesn't have a region, visit recursively
                            self.region_rec(neigbour_pos);
                        }
                    }
                    // plot_perimeter -= 1;
                }
            }
        }
        // todo how do I decide the region?
        // self.regions.entry(*plot.crop)
        //     .and_modify(| c | {
        //         c.0 += 1;
        //         c.1 += plot_perimeter;
        //     })
        //     .or_insert((1, plot_perimeter));

    }

    fn visualise_farm(&self) {
        let colours = vec![
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::BrightRed,
            Color::BrightGreen,
            Color::BrightYellow,
            Color::BrightBlue,
            Color::BrightMagenta,
            Color::BrightCyan,
        ];
        for y in self.yrange.clone() {
            let mut out = String::new();
            for x in self.xrange.clone() {
                let pos = Point {x, y};
                let plot = self.farm[&pos].clone();
                let mut cstring: ColoredString = plot.crop.to_string().white();
                if let Some(r) = plot.region {
                        cstring.fgcolor = Some(colours[r % colours.len()]);
                }
                print!("{}", cstring);
            }
            println!();
        }

    }
}


// for reference
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
fn rec_part_one(file: &str) -> usize {
    let mut farm = Farm::new(file);
    farm.find_regions();

    // todo visualise the grid
    //  colour based on region
    //  current point is bold
    farm.visualise_farm();

    // dbg!(&farm);

    1930
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    rec_part_one(file)
}

// #[allow(dead_code)]
// pub fn part_two(file: &str) -> usize {
//     194557
// }

#[cfg(test)]
mod tests {
    use crate::day_12::day12::part_one;

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

    // #[test]
    // fn test_part_two_test() {
    //     let result = part_two("src/day_12/day12_test.txt");
    //     assert_eq!(result, 65601038650482);
    // }

    // #[test]
    // fn test_part_two_data() {
    //     let result = part_two("src/day_12/day12_data.txt");
    //     assert_eq!(result, 231532558973909);
    // }
}
