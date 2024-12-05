pub fn process(input: &str) -> usize {
    0
}

mod grid {
    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum Direction {
        Right,
        Left,
        Up,
        Down,
        UpRight,
        DownRight,
        UpLeft,
        DownLeft,
    }

    /// Scans a line horizontally
    pub fn scan_for_pattern(
        input: &[&str],
        pattern: &str,
        direction: Direction,
    ) -> Result<u32, String> {
        let pattern_len = pattern.len();
        let num_rows = input.len();
        let num_columns = input
            .first()
            .expect("We should be provided at least one line.")
            .len();
        // Ensure all columns have equal length
        debug_assert!(input.par_iter().all(|row| row.len() == num_columns));

        // Verify that we have sufficient space to find the pattern in the desired direction.
        let enough_space = match direction {
            Direction::Right => num_columns >= pattern_len,
            _ => todo!("Add checks for remaining directions."),
        };

        if !enough_space {
            return Err("There isn't enough space available to find the pattern.".to_string());
        }

        let num_matches: u32 = match direction {
            Direction::Right => {
                // Scan each row, from start to end, looking for the pattern.
                input
                    .iter()
                    .map(|row| {
                        let mut num: u32 = 0;
                        // Scan the row for the string.
                        for index in 0..row.len() {
                            // Check if there is sufficien space to scan for the string.

                            // if (index as isize - row.len() as isize) < pattern_len as isize {
                            if usize::checked_sub(row.len(), index)
                                .is_some_and(|val| val < pattern_len)
                            {
                                // Not enough space to search for the string.
                                break;
                            }

                            // Extract the required number of elements to check.
                            let substring = &row[index..pattern_len];

                            if substring == pattern {
                                num += 1;
                            }
                        }

                        num
                    })
                    .sum()
            }
            _ => todo!("Handle remaining directions."),
        };

        Ok(num_matches)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{grid::scan_for_pattern, *};

    #[test]
    fn test_process() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(process(input), 18);
    }

    #[test]
    fn scan_for_pattern_with_the_pattern_once_returns_the_pattern() {
        let input = vec!["HELLO"];
        let num = scan_for_pattern(&input, "HELLO", grid::Direction::Right)
            .expect("Should find the string.");

        assert_eq!(num, 1);
    }

    #[rstest]
    #[case(vec!["HELLO"], Ok(1))]
    #[case(vec!["HELLO",
                "HELLO"], Ok(2))]
    #[case(vec!["HHELO"], Ok(0))]
    fn scan_for_pattern_test(#[case] input: Vec<&str>, #[case] expected: Result<u32, String>) {
        let num = scan_for_pattern(&input, "HELLO", grid::Direction::Right);

        assert_eq!(num, expected);
    }
}
