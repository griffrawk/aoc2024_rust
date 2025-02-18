#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::fs;

    // Part 1 structs & impl
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
                }
                if file {
                    file_id += 1
                };
            }
            let back = blocks.len() - 1;
            Disk {
                blocks,
                front,
                back,
            }
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

    // Part 2 structs & impl
    #[derive(Debug)]
    enum MapEntry {
        File { length: usize, file_id: usize },
        Gap { length: usize },
    }

    #[derive(Debug)]
    struct DiskMap {
        map_blocks: Vec<MapEntry>,
        front: usize,
        back: usize,
        last_file_id: usize,
    }

    impl DiskMap {
        fn new(file: &str) -> DiskMap {
            let mut map_blocks = Vec::new();
            let mut gap_map = HashMap::new();
            let front = 0;
            let mut file_id = 0;
            let mut last_file_id = 0;
            for (map_count, map_block) in fs::read_to_string(file)
                .expect("Can't read the file")
                .chars()
                .enumerate()
            {
                let file = map_count % 2 == 0;
                let length = map_block.to_digit(10).unwrap_or_default() as usize;
                if file {
                    map_blocks.push(MapEntry::File { length, file_id });
                    last_file_id = file_id;
                    file_id += 1;
                } else {
                    map_blocks.push(MapEntry::Gap { length });
                    gap_map
                        .entry(length)
                        .and_modify(|e: &mut Vec<usize>| e.push(map_count))
                        .or_insert(vec![map_count]);
                };
            }
            let back = map_blocks.len() - 1;

            DiskMap {
                map_blocks,
                front,
                back,
                last_file_id,
            }
        }

        fn compact_files(&mut self) {
            // Get rid of any consecutive Gaps in the input
            self.consolidate_gaps();
            let mut already_processed = self.last_file_id + 1;
            'backloop: while self.back > 0 {
                match self.map_blocks[self.back] {
                    // Process this file only if not previously done. As the back pointer can be reset, we need this to avoid
                    // moving files already discounted for move
                    MapEntry::File { length, file_id } if file_id < already_processed => {
                        let gap_needed = length;
                        self.front = 0;
                        while self.front < self.back {
                            match self.map_blocks[self.front] {
                                // Find a gap that fits
                                MapEntry::Gap { length } if length >= gap_needed => {
                                    // Note file_id for restart so this file is ignored on next pass
                                    already_processed = file_id;
                                    // Swap entries
                                    self.map_blocks[self.front] = MapEntry::File {
                                        length: gap_needed,
                                        file_id,
                                    };
                                    // Put a Gap where there used to be a File. Doesn't matter if it goes between existing Gaps
                                    // it will be consolidated in a microsec
                                    self.map_blocks[self.back] =
                                        MapEntry::Gap { length: gap_needed };
                                    // Check if we need to insert a gap
                                    let new_gap = length - gap_needed;
                                    if new_gap > 0 {
                                        self.map_blocks.insert(
                                            self.front + 1,
                                            MapEntry::Gap { length: new_gap },
                                        );
                                        self.consolidate_gaps();
                                        // Start again after consolidating gaps. As length (eg back) may have changed, it is no
                                        // longer guaranteed to be pointing to this file. Safer to start at the back and ignore
                                        // processed files based on checking the last file_id.
                                        continue 'backloop;
                                    }
                                    // Break from front loop
                                    break;
                                }
                                _ => {}
                            }
                            self.front += 1;
                        }
                    }
                    _ => {}
                }
                self.back -= 1;
            }
        }

        fn consolidate_gaps(&mut self) {
            let mut start = 0;
            let mut end = self.map_blocks.len() - 1;
            while start < end {
                match self.map_blocks[start] {
                    MapEntry::Gap { length: l1 } => {
                        // Look at the next block for a Gap
                        if let MapEntry::Gap { length: l2 } = self.map_blocks[start + 1] {
                            // Replace Gap with length = sum of Gaps lengths. Remove Gap + 1
                            self.map_blocks[start] = MapEntry::Gap { length: l1 + l2 };
                            self.map_blocks.remove(start + 1);
                            end -= 1;
                            // Now look for another Gap without moving forward
                            continue;
                        }
                    }
                    _ => {}
                }
                start += 1;
            }
            self.back = self.map_blocks.len() - 1;
        }

        fn checksum(&self) -> usize {
            let mut checksum = 0;
            // Keep track of virtual block position on disk
            let mut pos = 0;
            for entry in &self.map_blocks {
                match entry {
                    MapEntry::Gap { length } => {
                        // Increment virtual pos by gap length
                        pos += length;
                    }
                    MapEntry::File {
                        mut length,
                        file_id,
                    } => {
                        // Loop over virtual block positions and calc checksum for each block
                        while length > 0 {
                            checksum += file_id * pos;
                            pos += 1;
                            length -= 1;
                        }
                    }
                }
            }
            checksum
        }
    }

    // mains
    fn part_one(file: &str) -> usize {
        Disk::new(&file).compact_blocks()
    }

    fn part_two(file: &str) -> usize {
        let mut map = DiskMap::new(&file);
        map.compact_files();
        map.checksum()
    }

    #[test]
    fn test_part_one_test() {
        let result = part_one("src/bin/day_09/day09_test.txt");
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_one_data() {
        let result = part_one("src/bin/day_09/day09_data.txt");
        assert_eq!(result, 6519155389266);
    }

    #[test]
    fn test_part_two_test() {
        let result = part_two("src/bin/day_09/day09_test.txt");
        assert_eq!(result, 2858);
    }

    #[test]
    fn test_part_two_data() {
        let result = part_two("src/bin/day_09/day09_data.txt");
        assert_eq!(result, 6547228115826);
    }
}

fn main() {}
