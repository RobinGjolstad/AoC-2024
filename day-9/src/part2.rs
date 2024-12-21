#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiskSegment {
    capacity: usize,
    fill_level: usize,
    file: Vec<usize>,
    file_size: usize,
    free_space: Vec<usize>,
    free_space_size: usize,
}
impl DiskSegment {
    fn new(file_id: usize, file_size: usize, free_space: usize) -> Self {
        Self {
            capacity: file_size + free_space,
            fill_level: file_size,
            file: vec![file_id; file_size],
            file_size,
            free_space: Vec::with_capacity(free_space),
            free_space_size: free_space,
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
    let mut new_disk = segments.clone();
    let mut files_to_nuke = vec![];

    segments
        .iter()
        .enumerate()
        .rev()
        .for_each(|(segment_idx, segment)| {
            // Check if we can move the file into an earlier slot in the new disk.
            // If we can, we should move the file and remove the segment.
            for (new_disk_idx, new_disk_segment) in new_disk.iter_mut().enumerate() {
                // We should make sure we don't start moving elements from the start into the end.
                if segment_idx <= new_disk_idx {
                    // We're crossing over!
                    break;
                }
                if new_disk_segment.capacity - new_disk_segment.fill_level >= segment.fill_level {
                    // We can fit the segment into this segment.
                    // Push all the file slices from the segment into this segment.
                    segment.file.iter().for_each(|file_id| {
                        new_disk_segment.push_file_slice(*file_id);
                    });

                    files_to_nuke.push(segment_idx);

                    println!("{}: Moved file {}", new_disk_idx, segment_idx);
                    break;
                }
            }
        });

    files_to_nuke.sort();
    files_to_nuke.dedup();

    // Then we nuke the files.
    files_to_nuke.iter().rev().for_each(|idx| {
        let segment = new_disk
            .get_mut(*idx)
            .expect("We should always have a segment to nuke.");
        segment.fill_level -= segment.file.len();
        segment.file.clear();
    });

    // Calculate checksum.
    // Checksum is the overall index in the disk multiplied by the file-id at that index.
    // Each of these results are summed up.
    let disk_len = new_disk.len();
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
            // println!("Adding to checksum: {} * {}", overall_index, file_id);
            checksum += overall_index * file_id;
            overall_index += 1;
        }

        // If the file is empty it has been moved.
        // We should still update the overall index.
        if segment.file.is_empty() {
            // println!("Bumping overall index by: {}", segment.file_size);
            overall_index += segment.file_size;
        }

        for free_file_id in segment.free_space.iter() {
            // println!("Adding to checksum: {} * {}", overall_index, free_file_id);
            checksum += overall_index * free_file_id;
            overall_index += 1;
        }

        // Now we might have some free space.
        // We need to update the overall index even if there is no file slice.
        // println!(
        //     "Bumping overall index by: {}",
        //     segment.free_space_size - segment.free_space.len()
        // );
        overall_index += segment.free_space_size - segment.free_space.len();

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
        assert_eq!(process(input), 2858);
    }

    #[rstest]
    #[case(
        "11", 
        vec![
            DiskSegment{capacity: 2, fill_level: 1, file: vec![0], file_size: 1, free_space: vec![], free_space_size: 1}
        ]
    )]
    #[case(
        "1234", 
        vec![
            DiskSegment{capacity: 3, fill_level: 1, file: vec![0], file_size: 1, free_space: vec![], free_space_size: 2},
            DiskSegment{capacity: 7, fill_level: 3, file: vec![1, 1, 1], file_size: 3, free_space: vec![], free_space_size: 4}
        ]
    )]
    #[case(
        "12341234",
        vec![
            DiskSegment{capacity: 3, fill_level: 1, file: vec![0], file_size: 1, free_space: vec![], free_space_size: 2},
            DiskSegment{capacity: 7, fill_level: 3, file: vec![1, 1, 1], file_size: 3, free_space: vec![], free_space_size: 4},
            DiskSegment{capacity: 3, fill_level: 1, file: vec![2], file_size: 1, free_space: vec![], free_space_size: 2},
            DiskSegment{capacity: 7, fill_level: 3, file: vec![3, 3, 3], file_size: 3, free_space: vec![], free_space_size: 4}
        ]
    )]
    #[case(
        "2333133121414131402",
        vec![
            DiskSegment{capacity: 5, fill_level: 2, file: vec![0,0], file_size: 2, free_space: vec![], free_space_size: 3},
            DiskSegment{capacity: 6, fill_level: 3, file: vec![1,1,1], file_size: 3, free_space: vec![], free_space_size: 3},
            DiskSegment{capacity: 4, fill_level: 1, file: vec![2], file_size: 1, free_space: vec![], free_space_size: 3},
            DiskSegment{capacity: 4, fill_level: 3, file: vec![3,3,3], file_size: 3, free_space: vec![], free_space_size: 1},
            DiskSegment{capacity: 3, fill_level: 2, file: vec![4,4], file_size: 2, free_space: vec![], free_space_size: 1},
            DiskSegment{capacity: 5, fill_level: 4, file: vec![5,5,5,5], file_size: 4, free_space: vec![], free_space_size: 1},
            DiskSegment{capacity: 5, fill_level: 4, file: vec![6,6,6,6], file_size: 4, free_space: vec![], free_space_size: 1},
            DiskSegment{capacity: 4, fill_level: 3, file: vec![7,7,7], file_size: 3, free_space: vec![], free_space_size: 1},
            DiskSegment{capacity: 4, fill_level: 4, file: vec![8,8,8,8], file_size: 4, free_space: vec![], free_space_size: 0},
            DiskSegment{capacity: 2, fill_level: 2, file: vec![9,9], file_size: 2, free_space: vec![], free_space_size: 0}
        ]
    )]
    fn test_parse_line_segments(#[case] input: &str, #[case] expected: Vec<DiskSegment>) {
        let segments = parse_line_segments(input);
        assert_eq!(segments, expected);
    }
}

