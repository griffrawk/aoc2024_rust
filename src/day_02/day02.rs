use std::cmp::{max, min};
use std::fs;
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
    for report in reports.reports {
        for removal in -1 .. (report.len() as i32) {
            let mut clone = report.clone();
            if removal > -1 {
                clone.remove(removal as usize);
            }
            if safe_report(clone) {
                safe += 1;
                break;
            }
        }
    }
    safe
}

pub fn safe_report(report: Vec<i32>) -> bool {
    // Joost's method, but changed all to any so the bools truly represent
    // whether any increase or decrease found, rather than having to negate them
    // when checked.
    let l1 = &report[0 .. report.len()-1];
    let l2 = &report[1 .. report.len()];
    let increasing = zip(l1, l2).any(|a| a.0 < a.1);
    let decreasing = zip(l1, l2).any(|a| a.0 > a.1);
    if increasing & decreasing {
        return false
    }
    for (a, b) in zip(l1, l2) {
        let d = (a - b).abs();
        if (d < 1 ) | (d > 3 ) {
            return false
        }
    }
    true
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
        assert_eq!(result, 324);
    }
}
