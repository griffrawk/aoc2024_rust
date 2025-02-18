mod day15_in_bevy;
use crate::day15_in_bevy::*;

fn main() {
    bevy_main();
}

// AoC tests
pub mod day15;
#[cfg(test)]
mod tests {
    use crate::day15::part_one;

    #[test]
    fn test_part_one_basic() {
        let result = part_one("src/bin/day_15/day15_basic.txt");
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/bin/day_15/day15_test.txt");
        assert_eq!(result, 10092);
    }

    #[test]
    #[ignore = "Best run with --profile release, takes a long time"]
    fn test_part_one_data() {
        let result = part_one("src/bin/day_15/day15_data.txt");
        assert_eq!(result, 1421727);
    }
}
