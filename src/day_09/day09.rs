use std::fs;

#[derive(Debug)]
struct Disk {
    blocks: Vec<Option<usize>>,
    front: usize,
    back: usize,
}

impl Disk {
    fn new(file: &str) -> Disk {
        let mut blocks = Vec::new();
        let front = 0;
        let mut file_id = 0;
        for (map_count, map_block) in fs::read_to_string(file)
            .expect("Can't read the file")
            .chars()
            .enumerate()
        {
            let file = map_count % 2 == 0;
            for _ in 0..map_block.to_digit(10).unwrap_or_default() {
                if file {
                    // file
                    blocks.push(Some(file_id));
                  } else {
                    // free space
                    blocks.push(None);
                };
            };
            if file { file_id += 1 };
        }
        let back = blocks.len();
        Disk { blocks, front, back }
    }

    fn compact(&self) -> usize {
        let mut checksum = 1928;
        while self.front < self.back {

        }

        checksum
    }
}

#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    let disk = Disk::new(&file);
    disk.compact()
}

#[cfg(test)]
mod tests {
    use crate::day_09::day09::part_one;

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_09/day09_test.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_09/day09_data.txt");
        assert_eq!(result, 369);
    }
}
