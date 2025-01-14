use std::fs;
use MapEntry::{Gap, File};

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
        let back = blocks.len() - 1;
        Disk { blocks, front, back }
    }

    fn compact_blocks(&mut self) -> usize {
        // calc a checksum based on what the state would be if the data _did_ move!
        // data doesn't go anywhere...
        let mut checksum = 0;
        while self.front <= self.back {
            match self.blocks[self.front] {
                Some(file_id) => checksum += file_id * self.front,
                None => {
                    // work backward from end to find a block to fill free space at front
                    while self.back > self.front {
                        match self.blocks[self.back] {
                            Some(file_id) => {
                                checksum += file_id * self.front;
                                self.back -= 1;
                                break;
                            }
                            None => {}
                        }
                        self.back -= 1;
                    }
                }
            }
            self.front += 1;
        }

        checksum
    }
}

// part 2

#[derive(Debug)]
enum MapEntry {
    File { length: u32, file_id: Option<usize>, moved: bool },
    Gap { length: u32 },
}

#[derive(Debug)]
struct DiskMap {
    map_blocks: Vec<MapEntry>,
    front: usize,
    back: usize,
}

impl DiskMap {
    fn new(file: &str) -> DiskMap {
        let mut map_blocks = Vec::new();
        let front = 0;
        let mut file_id = 0;
        for (map_count, map_block) in fs::read_to_string(file)
            .expect("Can't read the file")
            .chars()
            .enumerate()
        {
            let file = map_count % 2 == 0;
            let length = map_block.to_digit(10).unwrap_or_default();
            if file {
                // file
                map_blocks.push(File { length, file_id: Some(file_id), moved: false });
            } else {
                // free space
                map_blocks.push(Gap { length });
            };
            if file { file_id += 1 };
        }
        let back = map_blocks.len() - 1;

        DiskMap { map_blocks, front, back }
    }

    fn compact_files(&mut self) -> usize {
        dbg!(&self.front, &self.back);
        self.checksum()
    }

    fn checksum(&self) -> usize {

        999
    }
}

// mains
#[allow(dead_code)]
pub fn part_one(file: &str) -> usize {
    Disk::new(&file).compact_blocks()
}

#[allow(dead_code)]
pub fn part_two(file: &str) -> usize {
    DiskMap::new(&file).compact_files()
}

#[cfg(test)]
mod tests {
    use crate::day_09::day09::{part_one, part_two};

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/day_09/day09_test.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/day_09/day09_data.txt");
        assert_eq!(result, 6519155389266);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/day_09/day09_test.txt");
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/day_09/day09_data.txt");
        assert_eq!(result, 999);
    }
}
