use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use aocutils::point::Point;
use colored::*;

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
                // exhausted region possibilities of pos, increment region
                self.current_region += 1;
            }
        }
    }

    fn region_rec(&mut self, pos: Point<i32>) {
        // return conditions
        let plot = self.farm[&pos].clone();
        if let Some(_) = plot.region {
            return
        }

        // process plot to find its region, and sumup the region's area & perimeter
        // assume a standalone crop has perimeter = 4
        let mut plot_perimeter = 4;

        // does plot have neighbours of same crop?
        for neigbour_pos in pos.cardinal_points() {
            if self.xrange.contains(&neigbour_pos.x) && self.yrange.contains(&neigbour_pos.y) {
                let neighbour_plot = self.farm[&neigbour_pos].clone();
                // same crop?
                if neighbour_plot.crop == plot.crop {
                    // -1 for each same neighbour crop
                    plot_perimeter -= 1;

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
                }

            }
        }
        // post-processing

        // if plot_perimeter is still = 4 the plot has no neighbours, but needs
        // reporting as a region of its own
        if plot_perimeter == 4 {
            // use new region and update pos
            self.farm.entry(pos)
                .and_modify(|p| p.region = Some(self.current_region));
        }

        // update region area & perimeter with a new clone of plot
        let plot = self.farm[&pos].clone();
        self.regions.entry(plot.region.unwrap())
            .and_modify(| c | {
                c.0 += 1;
                c.1 += plot_perimeter;
            })
            .or_insert((1, plot_perimeter));
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
            for x in self.xrange.clone() {
                let pos = Point {x, y};
                let plot = self.farm[&pos].clone();
                let mut cstring = plot.crop.to_string().white();
                if let Some(r) = plot.region {
                        cstring.fgcolor = Some(colours[r % colours.len()]);
                }
                print!("{}", cstring);
            }
            println!();
        }

    }
}

#[allow(dead_code)]
fn part_one(file: &str) -> usize {
    let mut farm = Farm::new(file);
    farm.find_regions();
    farm.visualise_farm();
    farm.regions.iter().map(|(_, (area, perimeter))| area * perimeter).sum()
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
        assert_eq!(result, 1449902);
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
