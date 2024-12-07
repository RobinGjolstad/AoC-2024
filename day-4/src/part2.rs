pub fn process(input: &str) -> usize {
    let input: Vec<&str> = input.lines().collect();

    let mut num_matches: usize = 0;

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, col_ch) in row.chars().enumerate() {
            // We just scan for 'A' and check the corners around it.
            if col_ch == 'A' {
                // Check each corner.
                let upper_left_coord = match (row_idx.checked_sub(1), col_idx.checked_sub(1)) {
                    (Some(r_i), Some(c_i)) => (r_i, c_i),
                    _ => continue,
                };
                let upper_right_coord = match (row_idx.checked_sub(1), col_idx.checked_add(1)) {
                    (Some(r_i), Some(c_i)) => (r_i, c_i),
                    _ => continue,
                };
                let lower_left_coord = match (row_idx.checked_add(1), col_idx.checked_sub(1)) {
                    (Some(r_i), Some(c_i)) => (r_i, c_i),
                    _ => continue,
                };
                let lower_right_coord = match (row_idx.checked_add(1), col_idx.checked_add(1)) {
                    (Some(r_i), Some(c_i)) => (r_i, c_i),
                    _ => continue,
                };

                // Now we can extract the corners.
                // Direct access is not safe, so we need to check if the extracted values exist.
                let upper_left = input
                    .get(upper_left_coord.0)
                    .and_then(|row| row.chars().nth(upper_left_coord.1));
                let upper_right = input
                    .get(upper_right_coord.0)
                    .and_then(|row| row.chars().nth(upper_right_coord.1));
                let lower_left = input
                    .get(lower_left_coord.0)
                    .and_then(|row| row.chars().nth(lower_left_coord.1));
                let lower_right = input
                    .get(lower_right_coord.0)
                    .and_then(|row| row.chars().nth(lower_right_coord.1));

                // Check if all corners are Some.
                if upper_left.is_none()
                    || upper_right.is_none()
                    || lower_left.is_none()
                    || lower_right.is_none()
                {
                    // Not all corners are present.
                    continue;
                }

                let upper_left = upper_left.unwrap();
                let upper_right = upper_right.unwrap();
                let lower_left = lower_left.unwrap();
                let lower_right = lower_right.unwrap();

                // Check each diagonal to verify that they are either MAS or SAM.
                let up_left_down_right = format!("{}{}{}", upper_left, col_ch, lower_right);
                let up_right_down_left = format!("{}{}{}", upper_right, col_ch, lower_left);

                if up_left_down_right == "MAS" || up_left_down_right == "SAM" {
                    // We have a match.
                    // Now we need to check the other diagonal.
                    if up_right_down_left == "MAS" || up_right_down_left == "SAM" {
                        // We have a match.
                        // Add to the number of matches.
                        num_matches += 1;
                    }
                }
            }
        }
    }

    num_matches
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

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
        9
    )]
    #[case(
        r#".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#,
        9
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}
