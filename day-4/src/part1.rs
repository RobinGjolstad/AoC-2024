pub fn process(input: &str) -> usize {
    let input: Vec<&str> = input.lines().collect();
    let right_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::Right)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&right_matches);
    let left_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::Left)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&left_matches);
    let down_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::Down)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&down_matches);
    let up_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::Up)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&up_matches);
    let up_right_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::UpRight)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&up_right_matches);
    let up_left_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::UpLeft)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&up_left_matches);
    let down_left_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::DownLeft)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&down_left_matches);
    let down_right_matches = grid::scan_for_pattern(&input, "XMAS", grid::Direction::DownRight)
        .expect("Input should be sufficiently sized to scan.");
    dbg!(&down_right_matches);

    (right_matches
        + left_matches
        + down_matches
        + up_matches
        + up_right_matches
        + up_left_matches
        + down_left_matches
        + down_right_matches)
        .try_into()
        .expect("The number of matches should be able to fit into a usize.")
}

mod grid {
    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GridError {
        InsufficientSpace,
    }

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
    ) -> Result<u32, GridError> {
        let num_rows = input.len();
        let num_columns = input
            .first()
            .expect("We should be provided at least one line.")
            .len();
        // Ensure all columns have equal length
        debug_assert!(input.par_iter().all(|row| row.len() == num_columns));

        // Verify that we have sufficient space to find the pattern in the desired direction.
        let enough_space = match direction {
            Direction::Right | Direction::Left => num_columns >= pattern.len(),
            Direction::Down | Direction::Up => num_rows >= pattern.len(),
            Direction::UpRight | Direction::UpLeft | Direction::DownRight | Direction::DownLeft => {
                num_rows >= pattern.len() && num_columns >= pattern.len()
            }
        };

        if !enough_space {
            return Err(GridError::InsufficientSpace);
        }

        let num_matches: u32 = match direction {
            Direction::Right => scan_for_pattern_right(input, pattern),
            Direction::Left => scan_for_pattern_left(input, pattern),
            Direction::Down => scan_for_pattern_down(input, pattern),
            Direction::Up => scan_for_pattern_up(input, pattern),
            Direction::UpRight => scan_for_pattern_up_right(input, pattern),
            Direction::UpLeft => scan_for_pattern_up_left(input, pattern),
            Direction::DownLeft => scan_for_pattern_down_left(input, pattern),
            Direction::DownRight => scan_for_pattern_down_right(input, pattern),
        };

        Ok(num_matches)
    }

    /// Scan each row, from start to end, looking for the pattern, looking "right".
    fn scan_for_pattern_right(input: &[&str], pattern: &str) -> u32 {
        input
            .iter()
            .enumerate()
            .map(|(row_i, row)| {
                let mut num: u32 = 0;
                // Scan the row for the string.
                for index in 0..row.len() {
                    // Check if there is sufficient space to scan for the string.

                    if usize::checked_sub(row.len(), index).is_some_and(|val| val < pattern.len()) {
                        // Not enough space to search for the string.
                        break;
                    }

                    // Extract the required number of elements to check.
                    let substring = &row[index..(index + pattern.len())];

                    if substring == pattern {
                        num += 1;
                        println!("Scanning from ({},{}) found match right", row_i, index);
                    }
                }

                num
            })
            .sum()
    }

    fn scan_for_pattern_left(input: &[&str], pattern: &str) -> u32 {
        input
            .iter()
            .enumerate()
            .map(|(row_i, row)| {
                let mut num: u32 = 0;
                // Scan the row for the string.
                for index in 0..=row.len() {
                    // Check if there is sufficient space to scan for the string.

                    if index < pattern.len() {
                        // Not enough space to search for the string.
                        continue;
                    }

                    // Extract the required number of elements to check.
                    let substring: String =
                        row[(index - pattern.len())..index].chars().rev().collect();

                    if substring == pattern {
                        println!("Scanning from ({},{}) found match left", row_i, index);
                        num += 1;
                    }
                }

                num
            })
            .sum()
    }

    fn scan_for_pattern_down(input: &[&str], pattern: &str) -> u32 {
        let mut matches_found: u32 = 0;

        // Iterate through the input and gradually compare the columns
        for column in 0..input.len() {
            for row in 0..input.len() {
                // Check each row down a column and gradually compare it to the pattern.

                let mut pattern_matched = true;

                for (pat_idx, pat_ch) in pattern.chars().enumerate() {
                    if input
                        .get(row + pat_idx)
                        .is_some_and(|row| row.chars().nth(column).is_some_and(|ch| ch == pat_ch))
                    {
                        // Character matched.
                        // Continue to the next one.
                    } else {
                        // We didn't match when comparing with the pattern.
                        // Try to move on in a new row.
                        pattern_matched = false;
                        break;
                    }
                }

                if pattern_matched {
                    println!("Scanning from ({},{}) found match down", row, column);
                    matches_found += 1;
                }
            }
        }

        matches_found
    }

    fn scan_for_pattern_up(input: &[&str], pattern: &str) -> u32 {
        let mut matches_found: u32 = 0;

        // Iterate through the input and gradually compare the columns
        for column in 0..input.len() {
            for row in 0..input.len() {
                // Check each row down a column and gradually compare it to the pattern.

                let mut pattern_matched = true;

                for (pat_idx, pat_ch) in pattern.chars().enumerate() {
                    let row_idx = match usize::checked_sub(row, pat_idx) {
                        Some(v) => v,
                        None => continue, // Out of bounds.
                    };
                    let col_idx = column;

                    if input
                        .get(row_idx)
                        .is_some_and(|row| row.chars().nth(col_idx).is_some_and(|ch| ch == pat_ch))
                    {
                        // Character matched.
                        // Continue to the next one.
                    } else {
                        // We didn't match when comparing with the pattern.
                        // Try to move on in a new row.
                        pattern_matched = false;
                        break;
                    }
                }

                if pattern_matched {
                    println!("Scanning from ({},{}) found match up", row, column);
                    matches_found += 1;
                }
            }
        }

        matches_found
    }

    fn scan_for_pattern_up_right(input: &[&str], pattern: &str) -> u32 {
        let mut matches_found: u32 = 0;

        // Iterate through the input and gradually compare the columns
        for column in 0..input.len() {
            for row in 0..input.len() {
                // Check each row down a column and gradually compare it to the pattern.

                let mut pattern_matched = true;

                for (pat_idx, pat_ch) in pattern.chars().enumerate() {
                    let row_idx = match usize::checked_sub(row, pat_idx) {
                        Some(v) => v,
                        None => continue, // Out of bounds.
                    };
                    let col_idx = column + pat_idx;
                    if input
                        .get(row_idx)
                        .is_some_and(|row| row.chars().nth(col_idx).is_some_and(|ch| ch == pat_ch))
                    {
                        // Character matched.
                        // Continue to the next one.
                    } else {
                        // We didn't match when comparing with the pattern.
                        // Try to move on in a new row.
                        pattern_matched = false;
                        break;
                    }
                }

                if pattern_matched {
                    println!("Scanning from ({},{}) found match up-right", row, column);
                    matches_found += 1;
                }
            }
        }

        matches_found
    }

    fn scan_for_pattern_up_left(input: &[&str], pattern: &str) -> u32 {
        let mut matches_found: u32 = 0;

        // Iterate through the input and gradually compare the columns
        for column in 0..input.len() {
            for row in 0..input.len() {
                // Check each row down a column and gradually compare it to the pattern.

                let mut pattern_matched = true;

                for (pat_idx, pat_ch) in pattern.chars().enumerate() {
                    let row_idx = match usize::checked_sub(row, pat_idx) {
                        Some(v) => v,
                        None => continue, // Out of bounds.
                    };
                    let col_idx = match usize::checked_sub(column, pat_idx) {
                        Some(v) => v,
                        None => continue, // Out of bounds
                    };
                    if input
                        .get(row_idx)
                        .is_some_and(|row| row.chars().nth(col_idx).is_some_and(|ch| ch == pat_ch))
                    {
                        // Character matched.
                        // Continue to the next one.
                    } else {
                        // We didn't match when comparing with the pattern.
                        // Try to move on in a new row.
                        pattern_matched = false;
                        break;
                    }
                }

                if pattern_matched {
                    println!("Scanning from ({},{}) found match up-left", row, column);
                    matches_found += 1;
                }
            }
        }

        matches_found
    }

    fn scan_for_pattern_down_left(input: &[&str], pattern: &str) -> u32 {
        let mut matches_found: u32 = 0;

        // Iterate through the input and gradually compare the columns
        for column in 0..input.len() {
            for row in 0..input.len() {
                // Check each row down a column and gradually compare it to the pattern.

                let mut pattern_matched = true;

                for (pat_idx, pat_ch) in pattern.chars().enumerate() {
                    let row_idx = row + pat_idx;
                    let col_idx = match usize::checked_sub(column, pat_idx) {
                        Some(v) => v,
                        None => continue, // Out of bounds
                    };
                    if input
                        .get(row_idx)
                        .is_some_and(|row| row.chars().nth(col_idx).is_some_and(|ch| ch == pat_ch))
                    {
                        // Character matched.
                        // Continue to the next one.
                    } else {
                        // We didn't match when comparing with the pattern.
                        // Try to move on in a new row.
                        pattern_matched = false;
                        break;
                    }
                }

                if pattern_matched {
                    println!("Scanning from ({},{}) found match down-left", row, column);
                    matches_found += 1;
                }
            }
        }

        matches_found
    }

    fn scan_for_pattern_down_right(input: &[&str], pattern: &str) -> u32 {
        let mut matches_found: u32 = 0;

        // Iterate through the input and gradually compare the columns
        for column in 0..input.len() {
            for row in 0..input.len() {
                // Check each row down a column and gradually compare it to the pattern.

                let mut pattern_matched = true;

                for (pat_idx, pat_ch) in pattern.chars().enumerate() {
                    let row_idx = row + pat_idx;
                    let col_idx = column + pat_idx;
                    if input
                        .get(row_idx)
                        .is_some_and(|row| row.chars().nth(col_idx).is_some_and(|ch| ch == pat_ch))
                    {
                        // Character matched.
                        // Continue to the next one.
                    } else {
                        // We didn't match when comparing with the pattern.
                        // Try to move on in a new row.
                        pattern_matched = false;
                        break;
                    }
                }

                if pattern_matched {
                    println!("Scanning from ({},{}) found match down-right", row, column);
                    matches_found += 1;
                }
            }
        }

        matches_found
    }

    /// Rotate a square grid 90 degrees clockwise.
    ///
    /// #Panics
    /// This function will panic if the input is not evenly sized.
    /// The number of rows and columns must be equal.
    fn rotate(input: &[&str]) -> Vec<String> {
        debug_assert_eq!(
            input.len(),
            input
                .first()
                .expect("The input should have at least one element")
                .len()
        );

        let mut rotated_input: Vec<Vec<char>> = vec![vec![' '; input.len()]; input.len()];
        (0..input.len()).for_each(|i| {
            for j in 0..input.len() {
                rotated_input[i][j] = input[input.len() - j - 1].chars().nth(i).expect("We should never be able to get out of bounds when iterating over the input length.");
            }
        });

        // Now convert each inner vec to a string.
        let rotated_input: Vec<String> = rotated_input
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect();

        rotated_input
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{grid::*, *};

    #[rstest]
    #[case(
        r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#,
        18
    )]
    #[case(
        r#"....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#,
        18
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }

    #[rstest]
    #[case(vec!["HELLO"], Ok(1))]
    #[case(vec!["HELLO",
                "HELLO"], Ok(2))]
    #[case(vec!["HHELO"], Ok(0))]
    #[case(vec!["afosdkj123HELLOadsfal"], Ok(1))]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    fn scan_for_pattern_right(#[case] input: Vec<&str>, #[case] expected: Result<u32, GridError>) {
        let num = scan_for_pattern(&input, "HELLO", Direction::Right);

        assert_eq!(num, expected);
    }

    #[rstest]
    #[case(vec!["OLLEH"], Ok(1))]
    #[case(vec!["OLLEH",
                "OLLEH"], Ok(2))]
    #[case(vec!["HELLO"], Ok(0))]
    #[case(vec!["afosOLLEH23HELLOaOLLEH"], Ok(2))]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    fn scan_for_pattern_left(#[case] input: Vec<&str>, #[case] expected: Result<u32, GridError>) {
        let num = scan_for_pattern(&input, "HELLO", Direction::Left);

        assert_eq!(num, expected);
    }

    #[rstest]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    #[case(vec![
            "H",
            "E",
            "L",
            "L",
            "O"], Ok(1))]
    #[case(vec![
            "H****",
            "E****",
            "L****",
            "L****",
            "O****"], Ok(1))]
    #[case(vec![
            "*",
            "*",
            "*",
            "H",
            "E",
            "L",
            "L",
            "O",
            "*",
            "*"], Ok(1))]
    #[case(vec![
            "*****",
            "*****",
            "H***H",
            "E***E",
            "L***L",
            "L*H*L",
            "O*E*O",
            "**L**",
            "H*L**",
            "E*O**",
            "L****",
            "L****",
            "O****",
            "*****",
            "*****"], Ok(4))]
    fn scan_for_pattern_down(#[case] input: Vec<&str>, #[case] expected: Result<u32, GridError>) {
        let num = scan_for_pattern(&input, "HELLO", Direction::Down);

        assert_eq!(num, expected);
    }

    #[rstest]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    #[case(vec![
            "O",
            "L",
            "L",
            "E",
            "H",
        ], Ok(1))]
    #[case(vec![
            "O****",
            "L****",
            "L****",
            "E****",
            "H****",
        ], Ok(1))]
    #[case(vec![
            "*",
            "*",
            "O",
            "L",
            "L",
            "E",
            "H",
            "*",
            "*",
            "*",
        ], Ok(1))]
    #[case(vec![
            "*****",
            "*****",
            "O****",
            "L****",
            "L****",
            "E*O**",
            "H*L**",
            "**L**",
            "O*E*O",
            "L*H*L",
            "L***L",
            "E***E",
            "H***H",
            "*****",
            "*****",
        ], Ok(4))]
    fn scan_for_pattern_up(#[case] input: Vec<&str>, #[case] expected: Result<u32, GridError>) {
        let num = scan_for_pattern(&input, "HELLO", Direction::Up);

        assert_eq!(num, expected);
    }

    #[rstest]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    #[case(vec![
        "****O",
        "***L*",
        "**L**",
        "*E***",
        "H****",
    ], Ok(1))]
    fn scan_for_pattern_up_right(
        #[case] input: Vec<&str>,
        #[case] expected: Result<u32, GridError>,
    ) {
        let num = scan_for_pattern(&input, "HELLO", Direction::UpRight);

        assert_eq!(num, expected);
    }

    #[rstest]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    #[case(vec![
        "O****",
        "*L***",
        "**L**",
        "***E*",
        "****H",
    ], Ok(1))]
    fn scan_for_pattern_up_left(
        #[case] input: Vec<&str>,
        #[case] expected: Result<u32, GridError>,
    ) {
        let num = scan_for_pattern(&input, "HELLO", Direction::UpLeft);

        assert_eq!(num, expected);
    }

    #[rstest]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    #[case(vec![
        "****H",
        "***E*",
        "**L**",
        "*L***",
        "O****",
    ], Ok(1))]
    fn scan_for_pattern_down_left(
        #[case] input: Vec<&str>,
        #[case] expected: Result<u32, GridError>,
    ) {
        let num = scan_for_pattern(&input, "HELLO", Direction::DownLeft);

        assert_eq!(num, expected);
    }

    #[rstest]
    #[case(vec!["a"], Err(GridError::InsufficientSpace))]
    #[case(vec![
        "H****",
        "*E***",
        "**L**",
        "***L*",
        "****O",
    ], Ok(1))]
    fn scan_for_pattern_down_right(
        #[case] input: Vec<&str>,
        #[case] expected: Result<u32, GridError>,
    ) {
        let num = scan_for_pattern(&input, "HELLO", Direction::DownRight);

        assert_eq!(num, expected);
    }
}
