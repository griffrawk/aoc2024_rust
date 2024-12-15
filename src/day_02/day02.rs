use std::cmp::{max, min};
use std::fs;
use colored::Colorize;
use std::iter::zip;

#[derive(Debug)]
struct Reports {
    reports: Vec<Vec<i32>>,
}

impl Reports {
    fn new(file: &str) -> Self {
        let contents: String = fs::read_to_string(file).expect("Can't read the file");
        let mut reports: Vec<Vec<i32>> = Vec::new();
        // chop up string slices into 2d 'array' of i32
        for line in contents.lines() {
            let values = line
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            reports.push(values);
        }
        Self { reports }
    }
}

#[allow(dead_code)]
pub fn part_one_original(file: &str) -> i32 {
    let reports = Reports::new(file);
    // println!("{:?}", reports);
    let mut safe = 0;
    for report in reports.reports {
        let mut first = true;
        let mut previous = 0;
        let mut maxd = 0;
        let mut mind = 0;
        'inner: {
            for level in report {
                if first {
                    first = false;
                } else {
                    let diff = previous - level;
                    // unsafe report if no diff, or too big
                    if (diff.abs() == 0) | (diff.abs() > 3) {
                        break 'inner;
                    }
                    maxd = max(maxd, diff);
                    mind = min(mind, diff);
                }
                previous = level;
            }
            // There can only be increase in one direction, and must be
            // increase in at least one direction. Unsafe report otherwise
            if ((maxd > 0) & (mind == 0)) | ((mind < 0) & (maxd == 0)) {
                safe += 1
            }
        }
    }
    safe
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> i32 {
    let reports = Reports::new(file);
    let mut safe = 0;
    for report in reports.reports {
        if safe_report(report) { safe += 1 }
    }
    safe
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> i32 {
    let reports = Reports::new(file);
    let mut safe = 0;
    let mut count = -1;
    for report in reports.reports {
        count += 1;
        println!("{} {:?}", count, report);
        for removal in -1..(report.len() as i32) - 1 {
            let mut clone = report.clone();
            if removal > -1 {
                clone.remove(removal as usize);
            }
            if safe_report(clone) {
                if removal > -1 {
                    println!(" - safe when level {} removed", report[removal as usize]);
                } else {
                    println!(" - safe");
                }
                safe += 1;
                break;
            }
        }
    }
    safe
}

pub fn safe_report(report: Vec<i32>) -> bool {
    // Joost's method
    let l1 = &report[0..report.len()-1];
    let l2 = &report[1..report.len()];
    let increasing = !zip(l1, l2).all(|a| a.0 <= a.1);
    let decreasing = !zip(l1, l2).all(|a| a.0 >= a.1);
    if increasing & decreasing { return false }
    for (a, b) in zip(l1, l2) {
        let d = (a - b).abs();
        if (d < 1 ) | (d > 3 ) { return false }
    }
    true
}

#[allow(dead_code)]
pub fn part_two_original(file: &str) -> i32 {
    let reports = Reports::new(file);
    let mut safe = 0;
    let mut count = 0;
    for report in reports.reports {
        let mut first = true;
        let mut previous = 0;
        let mut increase = false;
        let mut decrease = false;
        let mut unsafe_count = 0;
        count += 1;
        println!("{} {:?}", count, report);
        for level in report {
            'nextlevel: {
                let mut this_increase = false;
                let mut this_decrease = false;
                let mut this_unsafe = 0;
                if first {
                    first = false;
                } else {
                    println!("{} {} {} {}", "- previous:".cyan(), previous, "level:".cyan(), level);
                    let diff = previous - level;
                    match diff {
                        ..-3 | 4.. => {
                            println!("{} {} {} {}", "- diff out of bounds from".red(), previous, "to".red(), level);
                            this_unsafe += 1
                        },
                        -3..0 => this_increase = true,
                        0 => {
                            println!("{} {}", "- no diff at".red(), level);
                            this_unsafe += 1
                        },
                        1..=3 => this_decrease = true,
                    }
                    if increase & this_decrease {
                        this_decrease = false;
                        this_unsafe += 1;
                        println!("{} {} {} {}", "- increasing then decrease from".red(), previous, "to".red(), level);
                    }
                    if this_increase & decrease {
                        this_increase = false;
                        this_unsafe += 1;
                        println!("{} {} {} {}", "- decreasing then increase from".red(), previous, "to".red(), level);
                    }
                    // if this_unsafe, ignore this level so that the next level is compared to
                    // current previous. add this unsafe to overall for report
                    //
                    // However, this assumes that previous must be kept in all cases and that level is at fault. eg.
                    //
                    // 714 [76, 72, 68, 67, 65, 62, 56]
                    // - previous: 76 level: 72
                    // - diff out of bounds from 76 to 72
                    // - previous: 76 level: 68
                    // - diff out of bounds from 76 to 68
                    // - previous: 76 level: 67
                    // - diff out of bounds from 76 to 67
                    // - previous: 76 level: 65
                    // - diff out of bounds from 76 to 65
                    // - previous: 76 level: 62
                    // - diff out of bounds from 76 to 62
                    // - previous: 76 level: 56
                    // - diff out of bounds from 76 to 56
                    // Unsafe
                    //
                    // 72 onwards are all ok, but 76 is the outlier. But in current code, 76 is retained and everything
                    // else is rejected. A Principal Skinner moment...

                    unsafe_count += this_unsafe;
                    if this_unsafe > 0 { break 'nextlevel }
                }
                previous = level;
                increase = this_increase;
                decrease = this_decrease;
            }
        }
        if unsafe_count == 1 {
            println!("{}", "Ignored".yellow());
        }
        if unsafe_count <= 1 {
            println!("{}", "Safe".bright_green());
            safe += 1;
        } else {
            println!("{}", "Unsafe".bright_red());
        }

    }
    safe
}

pub fn part_two_alt() {

}

#[cfg(test)]
mod tests {
    use crate::day_02::day02::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_02/day02_test.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_02/day02_data.txt");
        assert_eq!(result, 252);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_02/day02_test.txt");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_02/day02_data.txt");
        assert_eq!(result, 21306195);
    }
}
