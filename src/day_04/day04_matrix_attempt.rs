use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;
// use regex::Regex;

#[allow(dead_code)]
#[derive(Debug)]
struct Rotated {
    rxmin: i32,
    rxmax: i32,
    rymin: i32,
    rymax: i32,
    rotation: HashMap<(i32, i32), (i32, i32)>
}

impl Rotated {
    fn new() -> Self {
        let rxmin= 0;
        let rxmax = 0;
        let rymin =  0;
        let rymax = 0;
        let rotation: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        Self { rxmin, rxmax, rymin, rymax, rotation }
    }
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> i32 {
    let mut res = 0;
    let contents = fs::read_to_string(file).expect("Can't read the file");
    let mut wordsearch: Vec<String> = Vec::new();
    for line in contents.lines() {
        wordsearch.push(line.to_string());

    }
    dbg!(&wordsearch);
    serial_rotated_read(wordsearch, 45);

    res
}
fn serial_rotated_read(input: Vec<String>, rotation: usize ) -> String {
    let xmax = input[0].len() as i32;
    let ymax = input.len() as i32;
    let rotated = rotate(0, xmax, 0, ymax, rotation);
    let size = max(xmax, ymax) * 4;
    dbg!(size);
    // create an initialised sized array to hold the translated input
    let mut translated = vec![vec!["."; size as usize]; size as usize];
    // dbg!(&translated);
    // populate array using the original data, translating the coords
    for ((x, y), (xn, yn)) in rotated.rotation.iter() {
        let x_old = *x as usize;
        let y_old = *y as usize;
        let x_new = (*xn + size / 2) as usize;
        let y_new = (*yn + size / 2) as usize;
        translated[y_new][x_new] = input[y_old].get(x_old..x_old+1).unwrap();
    }
    dbg!(&translated);
    let mut res: String = String::new();
    res.push(input[0].chars().nth(9).unwrap());
    res
}

/// ### Rotate coordinate ranges by angle around centre (0, 0)
/// xmin, xmax - inclusive x range
///
/// ymin, ymax - inclusive y range
///
/// angle - one of 0, 45, 90, 135 degrees
///
/// Returns HashMap of original (x,y) mapped to rotated coordinates. Values are rounded
/// up (+ve) or down (-ve), so the ranges will have gaps for rotations of 45, 135 degrees.
fn rotate(xmin: i32, xmax: i32, ymin: i32, ymax: i32, angle: usize) -> Rotated {
    let mut cosmul = 0.0;
    let mut sinmul = 0.0;
    // let mut res: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut res = Rotated::new();

    match angle {
        0   => { cosmul = 1.0; sinmul = 0.0; }
        45  => { cosmul = 0.7; sinmul = 0.7; }
        90  => { cosmul = 0.0; sinmul = 1.0; }
        135 => { cosmul = -0.7; sinmul = 0.7; }
        _   => panic!("Bad angle")
    }

    for x in xmin..xmax {
        for y in ymin..ymax {
            let mut xn = (x as f64 * cosmul) - (y as f64 * sinmul);
            let mut yn = (x as f64 * sinmul) + (y as f64 * cosmul);
            if xn.is_sign_negative() { xn = xn.floor()} else { xn = xn.ceil()}
            if yn.is_sign_negative() { yn = yn.floor()} else { yn = yn.ceil()}
            res.rxmin = min(res.rxmin, xn as i32);
            res.rxmax = max(res.rxmax, xn as i32);
            res.rymin = min(res.rymin, yn as i32);
            res.rymax = max(res.rymax, yn as i32);
            res.rotation.insert((x, y), (xn as i32, yn as i32));
        }
    }
    // dbg!(&res);
    res
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> i32 {

    0
}

#[cfg(test)]
mod tests {
    use crate::day_04::day04::{part_one, part_two, rotate};

    #[test]
    fn test_rotate() {
        dbg!(rotate(0, 1,0, 1, 45));
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_04/day04_test.txt");
        // assert_eq!(result, 18);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_04/day04_data.txt");
        assert_eq!(result, 161289189);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_04/day04_test.txt");
        assert_eq!(result, 48);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_04/day04_data.txt");
        assert_eq!(result, 83595109);
    }
}
