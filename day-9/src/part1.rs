#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiskSegment {
    capacity: usize,
    fill_level: usize,
    file: Vec<usize>,
    free_space: Vec<usize>,
}
impl DiskSegment {
    fn new(file_id: usize, file_size: usize, free_space: usize) -> Self {
        Self {
            capacity: file_size + free_space,
            fill_level: file_size,
            file: vec![file_id; file_size],
            free_space: Vec::with_capacity(free_space),
        }
    }

    /// Push a file slice into an empty slot.
    ///
    /// This function returns the pushed value if it succeeds.
    fn push_file_slice(&mut self, file_id: usize) -> Option<usize> {
        if self.fill_level + 1 > self.capacity {
            // We don't have any more free space.
            return None;
        }

        self.free_space.push(file_id);
        self.fill_level += 1;

        Some(file_id)
    }

    /// Pop a file slice from a segment.
    ///
    /// the free_space segment is popped first, then the file.
    fn pop_file_slice(&mut self) -> Option<usize> {
        if self.fill_level == 0 {
            return None;
        }

        if !self.free_space.is_empty() {
            self.fill_level -= 1;
            self.free_space.pop()
        } else if !self.file.is_empty() {
            self.fill_level -= 1;
            self.file.pop()
        } else {
            None
        }
    }
}

fn parse_line_segments(input: &str) -> Vec<DiskSegment> {
    // Collect all pairs into Vec<(u32, u32)>.
    // If we have an odd number of elements, we should add a 0 to the end.
    // We should then create a DiskSegment for each pair of elements.
    let pairs: Vec<(u32, u32)> = input
        .chars()
        .map(|c| {
            c.to_digit(10)
                .expect("We should always be able to parse numbers.")
        })
        .collect::<Vec<u32>>()
        .chunks(2)
        .map(|chunk| {
            if chunk.len() == 1 {
                (chunk[0], 0)
            } else {
                (chunk[0], chunk[1])
            }
        })
        .collect();

    let segments: Vec<DiskSegment> = pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| DiskSegment::new(idx, pair.0 as usize, pair.1 as usize))
        .collect();

    segments
}

pub fn process(input: &str) -> usize {
    let segments = parse_line_segments(input.trim());

    // Pop from end and push to front.
    let mut new_disk = segments.clone();
    let disk_len = new_disk.len();
    // Swoop through the new disk.
    // Pop from the last segment and push onto the first free slot.
    // Continue until there are no more free slots before the one we're currently popping from.
    let mut current_first_segment_index = 0;
    let mut current_last_segment_index = disk_len - 1;
    while current_first_segment_index < current_last_segment_index {
        let mut current_first_segment = new_disk
            .get(current_first_segment_index)
            .expect("We should always have a segment.")
            .clone();
        let mut current_last_segment = new_disk
            .get(current_last_segment_index)
            .expect("We should always have a segment.")
            .clone();

        // Move files from the last segment to the first segment.
        // Continue until we can't move any more files, either because the first segment is full or
        // the last segment is empty.

        while current_first_segment.fill_level < current_first_segment.capacity
            && current_last_segment.fill_level > 0
        {
            if let Some(file_id) = current_last_segment.pop_file_slice() {
                if current_first_segment.push_file_slice(file_id).is_some() {
                    // We successfully pushed the file slice.
                    // Continue to the next file slice.
                    continue;
                }

                // We couldn't push the file slice.
                // We should break out of the loop.
                break;
            }
        }

        // Update the segments.
        new_disk[current_first_segment_index] = current_first_segment;
        new_disk[current_last_segment_index] = current_last_segment;

        // Update the indexes.
        if new_disk[current_first_segment_index].fill_level
            == new_disk[current_first_segment_index].capacity
        {
            current_first_segment_index += 1;
        }
        if new_disk[current_last_segment_index].fill_level == 0 {
            current_last_segment_index -= 1;
        }
    }

    // Calculate checksum.
    // Checksum is the overall index in the disk multiplied by the file-id at that index.
    // Each of these results are summed up.
    let mut disk_index = 0;
    let mut overall_index = 0;
    let mut checksum = 0;
    loop {
        if disk_index >= disk_len {
            break;
        }

        let segment = new_disk
            .get(disk_index)
            .expect("We should always have a segment.");

        for file_id in segment.file.iter() {
            checksum += overall_index * file_id;
            overall_index += 1;
        }

        for free_file_id in segment.free_space.iter() {
            checksum += overall_index * free_file_id;
            overall_index += 1;
        }

        disk_index += 1;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!(process(input), 1928);
    }

    #[rstest]
    #[case(
        "11", 
        vec![
            DiskSegment{capacity: 2, fill_level: 1, file: vec![0], free_space: vec![]}
        ]
    )]
    #[case(
        "1234", 
        vec![
            DiskSegment{capacity: 3, fill_level: 1, file: vec![0], free_space: vec![]},
            DiskSegment{capacity: 7, fill_level: 3, file: vec![1, 1, 1], free_space: vec![]}
        ]
    )]
    #[case(
        "12341234",
        vec![
            DiskSegment{capacity: 3, fill_level: 1, file: vec![0], free_space: vec![]},
            DiskSegment{capacity: 7, fill_level: 3, file: vec![1, 1, 1], free_space: vec![]},
            DiskSegment{capacity: 3, fill_level: 1, file: vec![2], free_space: vec![]},
            DiskSegment{capacity: 7, fill_level: 3, file: vec![3, 3, 3], free_space: vec![]}
        ]
    )]
    #[case(
        "2333133121414131402",
        vec![
            DiskSegment{capacity: 5, fill_level: 2, file: vec![0,0], free_space: vec![]},
            DiskSegment{capacity: 6, fill_level: 3, file: vec![1,1,1], free_space: vec![]},
            DiskSegment{capacity: 4, fill_level: 1, file: vec![2], free_space: vec![]},
            DiskSegment{capacity: 4, fill_level: 3, file: vec![3,3,3], free_space: vec![]},
            DiskSegment{capacity: 3, fill_level: 2, file: vec![4,4], free_space: vec![]},
            DiskSegment{capacity: 5, fill_level: 4, file: vec![5,5,5,5], free_space: vec![]},
            DiskSegment{capacity: 5, fill_level: 4, file: vec![6,6,6,6], free_space: vec![]},
            DiskSegment{capacity: 4, fill_level: 3, file: vec![7,7,7], free_space: vec![]},
            DiskSegment{capacity: 4, fill_level: 4, file: vec![8,8,8,8], free_space: vec![]},
            DiskSegment{capacity: 2, fill_level: 2, file: vec![9,9], free_space: vec![]}
        ]
    )]
    fn test_parse_line_segments(#[case] input: &str, #[case] expected: Vec<DiskSegment>) {
        let segments = parse_line_segments(input);
        assert_eq!(segments, expected);
    }
}
