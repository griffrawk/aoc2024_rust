#[cfg(test)]
mod tests {

    use aocutils::point::Point;
    use colored::*;
    use std::collections::HashMap;
    use std::fs;
    use std::ops::Range;

    // Corner checking
    // ---------------
    // Constants for the corner check. The arrays are read L->R as N, NE, E, SE, S, SW, W, NW
    // around the pos being checked. This pos is not is the array, just the points around it.
    // Some(bool) should be matched against the compass points of the pos being examined,
    // - Some(true) is a pos in the same region as O
    // - Some(false) is a point in a neighbour region or out-of-bounds.
    // - None if a compass point isn't needed e.g. inside the region or behind the external corner

    // Internal corners e.g. int_se is :
    // FT_
    // TO_
    // ___
    //
    // gives:     N,          NE,   E,    SE,   S,    SW,   W,          NW
    //           [Some(true), None, None, None, None, None, Some(true), Some(false)]

    // External corners require some more care, where members of the same region touch diagonally
    // e.g. A region encloses B regions. For an ext_se check for A at 2,2 it has to ignore
    // the A at 3,3 and just detect Some(false) for the B's at 2,3 and 3,2, otherwise it
    // misses the corner.
    //
    // AAAAAA
    // AAABBA
    // AAABBA
    // ABBAAA
    // ABBAAA
    // AAAAAA

    // So... External corners e.g. ext_se is :
    // ___
    // _OF
    // _F_
    //
    // gives:     N,    NE,   E,           SE,   S,           SW,   W,    NW
    //           [None, None, Some(false), None, Some(false), None, None, None]

    const CORNERS: [[Option<bool>; 8]; 8] = [
        [
            None,
            None,
            None,
            None,
            Some(true),
            Some(false),
            Some(true),
            None,
        ], // int_ne
        [
            Some(true),
            None,
            None,
            None,
            None,
            None,
            Some(true),
            Some(false),
        ], // int_se
        [
            Some(true),
            Some(false),
            Some(true),
            None,
            None,
            None,
            None,
            None,
        ], // int_sw
        [
            None,
            None,
            Some(true),
            Some(false),
            Some(true),
            None,
            None,
            None,
        ], // int_nw
        [Some(false), None, Some(false), None, None, None, None, None], // ext_ne
        [None, None, Some(false), None, Some(false), None, None, None], // ext_se
        [None, None, None, None, Some(false), None, Some(false), None], // ext_sw
        [Some(false), None, None, None, None, None, Some(false), None], // ext_nw
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
        regions: HashMap<usize, (usize, usize, usize, char)>, // k: region v: (area, perimeter, corners, crop)
    }

    impl Farm {
        fn new(file: &str) -> Self {
            // Plots addressable by Point
            let mut farm: HashMap<Point<i32>, Plot> = HashMap::new();
            let mut max_x = 0;
            let mut max_y = 0;
            for (y, line) in fs::read_to_string(file)
                .expect("Can't read the file")
                .lines()
                .enumerate()
            {
                max_y = y as i32;
                for (x, c) in line.chars().enumerate() {
                    max_x = x as i32;
                    if c.is_ascii_alphanumeric() {
                        farm.insert(
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            Plot {
                                region: None,
                                crop: c,
                            },
                        );
                    }
                }
            }
            let regions: HashMap<usize, (usize, usize, usize, char)> = HashMap::new();
            Self {
                farm,
                xrange: 0..max_x + 1,
                yrange: 0..max_y + 1,
                current_region: 0,
                regions,
            }
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
                return;
            }

            // Process plot to find its region, and sum-up the region's area & perimeter
            // Assume a standalone crop has perimeter = 4
            let mut plot_perimeter = 4;
            // Always use new region. If plot has neighbours that will
            // be overwritten later. This provides a default for regions of one plot.
            self.farm
                .entry(pos)
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
                                self.farm.entry(pos).and_modify(|p| p.region = Some(r));
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

            self.regions
                .entry(plot.region.unwrap())
                .and_modify(|c| {
                    c.0 += 1; // Area
                    c.1 += plot_perimeter; // Perimeter
                })
                .or_insert((1, plot_perimeter, 0, plot.crop));
        }

        fn find_corners(&mut self) {
            // find corners for the sides
            for (pos, _) in self.farm.clone() {
                let corners = self.corners(pos);
                let plot = &self.farm[&pos];
                self.regions.entry(plot.region.unwrap()).and_modify(|c| {
                    c.2 += corners;
                });
            }
        }
        fn corners(&self, pos: Point<i32>) -> usize {
            // For a given Point, find if the Point is on the internal, or external
            // turn of a corner.
            // This will be done by checking the neighbours of the point and return
            // true if the patterns match various combinations
            // A pos can have 0, 1, 2 or 4 corners
            let mut corners = 0;
            let mut neighbour_matches: Vec<Option<bool>> = Vec::new();
            // N, NE, E, SE, S, SW, W, NW
            // Make a view of the compass points around pos
            // Some(true) for same region
            // Some(false) for different region, or out-of-bounds
            for neighbour in pos.compass_points() {
                if self.xrange.contains(&neighbour.x) && self.yrange.contains(&neighbour.y) {
                    if self.farm[&neighbour].region == self.farm[&pos].region {
                        neighbour_matches.push(Some(true));
                    } else {
                        neighbour_matches.push(Some(false));
                    }
                } else {
                    // out-of-bounds equates to false
                    neighbour_matches.push(Some(false));
                }
            }
            // Now compare each CORNERS pattern check with the view of pos
            // If pattern check has a None, that neighbour is passed
            // If pattern gets to pass == 8, the pos has a corner
            for pattern in 0..8 {
                let mut pass = 0;
                for check in 0..8 {
                    match CORNERS[pattern][check] {
                        Some(r) => {
                            if r == neighbour_matches[check].unwrap() {
                                pass += 1;
                            }
                        }
                        None => pass += 1,
                    }
                }
                if pass == 8 {
                    corners += 1
                }
            }
            corners
        }

        fn visualise_farm(&self) {
            // Attempt to visualise the farm as a coloured map. Unaware of the
            // 4-colour problem, it can output regions with touching similar colours
            // 36 combinations aren't enough it seems...
            let colours = [
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
                            _ => (),
                        }
                    }
                    print!("{}", cstring);
                }
                println!();
            }
            println!();
        }
    }

    fn part_one_two(file: &str) -> (usize, usize) {
        let mut farm = Farm::new(file);
        farm.find_regions();
        farm.visualise_farm();
        farm.find_corners();
        (
            farm.regions
                .iter()
                .map(|(_, (area, perimeter, _, _))| area * perimeter)
                .sum(),
            farm.regions
                .iter()
                .map(|(_, (area, _, corners, _))| area * corners)
                .sum(),
        )
    }

    #[test]
    fn single_region_test() {
        let mut farm = Farm::new("src/day_12/day12_test.txt");
        farm.region_rec(Point { x: 6, y: 0 });
        dbg!(&farm);
    }
    #[test]
    fn corner_test() {
        let mut farm = Farm::new("src/day_12/day12_test.txt");
        farm.find_regions();
        assert_eq!(farm.corners(Point { x: 2, y: 0 }), 0);
        assert_eq!(farm.corners(Point { x: 3, y: 0 }), 1);
        assert_eq!(farm.corners(Point { x: 5, y: 6 }), 2);
        assert_eq!(farm.corners(Point { x: 7, y: 4 }), 4);
        assert_eq!(farm.corners(Point { x: 7, y: 9 }), 1);
        assert_eq!(farm.corners(Point { x: 3, y: 3 }), 2);
        // farm corners. sanity check both bounds
        assert_eq!(farm.corners(Point { x: 0, y: 0 }), 1);
        assert_eq!(farm.corners(Point { x: 9, y: 0 }), 2);
        assert_eq!(farm.corners(Point { x: 0, y: 9 }), 2);
        assert_eq!(farm.corners(Point { x: 9, y: 9 }), 1);
    }

    // Puzzle example
    #[test]
    fn test_part_one_two_test() {
        let result = part_one_two("src/day_12/day12_test.txt");
        assert_eq!(result, (1930, 1206));
    }

    // Smaller farms for edge cases
    #[test]
    fn test_part_one_two_test_a() {
        let result = part_one_two("src/day_12/day12_2_test_a.txt");
        assert_eq!(result, (1184, 368));
    }

    #[test]
    fn test_part_one_two_test_b() {
        let result = part_one_two("src/day_12/day12_2_test_b.txt");
        assert_eq!(result, (692, 236));
    }

    // Puzzle data
    #[test]
    fn test_part_one_two_data() {
        let result = part_one_two("src/day_12/day12_data.txt");
        assert_eq!(result, (1449902, 908042));
    }
    // 916993 too high
}
