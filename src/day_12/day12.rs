use std::collections::HashMap;
use std::{fs, io};
use std::ops::Range;
use aocutils::point::Point;
use colored::*;

// Constants for the corner check. The arrays are read L->R as N, NE, E, SE, S, SW, W, NW
// around the pos being checked. This pos is not is the array, just the points around it.
// Some(bool) should be matched against the compass points of the pos being examined,
// - Some(true) is a pos in the current region
// - Some(false) is a point in a neighbour region or out-of-bounds.
// - None if a compass point isn't needed.

// Internal corners e.g. int_se is...
// YX_
// XX_
// ___

const CORNERS: [[Some<bool>;8];8] = [
    [Some(true), None, None, None, None, None, Some(true), Some(false)],    // int_se
    [Some(true), Some(false), Some(true), None, None, None, None, None],    // int_sw
    [None, None, Some(true), Some(false), Some(true), None, None, None],    // int_nw
    [None, None, None, None, Some(true), Some(false), Some(true), None],    // int_ne
    [Some(false), Some(false), Some(false), None, None, None, None, None],  // ext_ne
    [None, None, Some(false), Some(false), Some(false), None, None, None],  // ext_se
    [None, None, None, None, Some(false), Some(false), Some(false), None],  // ext_sw
    [Some(false), None, None, None, None, None, Some(false), Some(false)],  // ext_nw
    ];

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
        // Plots addressable by Point
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
        for (pos, _) in self.farm.clone() {
            // Only recurse into region-less plots
            if let None = self.farm[&pos].region {
                self.region_rec(pos);
                // Exhausted region possibilities of pos, so increment region
                self.current_region += 1;
            }
        }
    }

    fn region_rec(&mut self, pos: Point<i32>) {
        // Return conditions
        let plot = self.farm[&pos].clone();
        if let Some(_) = plot.region {
            return
        }

        // Process plot to find its region, and sum-up the region's area & perimeter
        // Assume a standalone crop has perimeter = 4
        let mut plot_perimeter = 4;
        // Always use new region. If plot has neighbours that will
        // be overwritten later. This provides a default for regions of one plot.
        self.farm.entry(pos)
            .and_modify(|p| p.region = Some(self.current_region));

        // todo use visualiser here for a frame-by-frame
        //  but need a way of animating the text output
        //  might need to be something animatable...
        // self.visualise_farm();

        // Does plot have neighbours of same crop?
        for neigbour_pos in pos.cardinal_points() {
            if self.xrange.contains(&neigbour_pos.x) && self.yrange.contains(&neigbour_pos.y) {
                let neighbour_plot = &self.farm[&neigbour_pos];
                // Same crop?
                if neighbour_plot.crop == plot.crop {
                    // -1 for each same neighbour crop
                    plot_perimeter -= 1;
                    // Already in a region?
                    match neighbour_plot.region {
                        Some(r) => {
                            // Set plot region to same as neighbour
                            self.farm.entry(pos)
                                .and_modify(|p| p.region = Some(r));
                        }
                        None => {
                            // Neighbour doesn't have a region, visit recursively
                            self.region_rec(neigbour_pos);
                        }
                    }
                }
            }
        }
        // Post-processing

        // Sum-up region area & perimeter
        let plot = &self.farm[&pos];
        self.regions.entry(plot.region.unwrap())
            .and_modify(| c | {
                c.0 += 1;                           // Area
                c.1 += plot_perimeter;              // Perimeter
            })
            .or_insert((1, plot_perimeter));
    }

    fn corner_finder(&self, pos: Point<i32>) -> bool {
        // For a given Point, find if the Point is on the internal, or external
        // turn of a corner.
        // This will be done by checking the neighbours of the point and return
        // true if the patterns match various combinations
        let neighbours = pos.compass_points();




        true
    }



    fn visualise_farm(&self) {
        // Attempt to visualise the farm as a coloured map. Unaware of the
        // 4-colour problem, it can output regions with touching similar colours
        // 36 combinations aren't enough it seems...
        let colours = vec![
            Color::Red,
            Color::Green,
            Color::Yellow,
            Color::Blue,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::BrightRed,
            Color::BrightGreen,
            Color::BrightYellow,
            Color::BrightBlue,
            Color::BrightMagenta,
            Color::BrightCyan,
            Color::BrightWhite,
        ];
        let italic = "Italic".italic();
        let bold = "Bold".bold();
        for y in self.yrange.clone() {
            for x in self.xrange.clone() {
                let plot = &self.farm[&Point { x, y }];
                let mut cstring: ColoredString = plot.crop.to_string().normal();
                if let Some(r) = plot.region {
                    cstring.fgcolor = Some(colours[r % colours.len()]);
                    match r % 3 {
                        0 => cstring.style = italic.style,
                        1 => cstring.style = bold.style,
                        _ => ()
                    }
                }
                print!("{}", cstring);
            }
            println!();
        }
        println!();
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
    use aocutils::point::Point;
    use crate::day_12::day12::{part_one, Farm};

    #[test]
    fn corner_test() {
        let farm = Farm::new("src/day_12/day12_test.txt");
        assert!(farm.corner_finder( Point {x: 3, y: 0 }));
    }

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
