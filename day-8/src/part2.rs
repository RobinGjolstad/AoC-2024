use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub fn process(input: &str) -> usize {
    // Get size of grid.
    let y = input.lines().filter(|line| !line.is_empty()).count();
    let x = input.lines().next().unwrap().chars().count();

    let antennas = parse_grid_manual(input);

    let sorted_antennas = sort_antennas(&antennas);
    let antinodes = find_antinodes(&sorted_antennas, (x as i32, y as i32));

    // Filter out any overlapping antinodes.
    let antinodes = antinodes
        .iter()
        .map(|(_, pos)| *pos)
        .collect::<HashSet<_>>();

    antinodes.len()
}

fn parse_grid_manual(input: &str) -> HashMap<IVec2, char> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch.is_alphanumeric() {
                grid.insert(IVec2::new(x as i32, y as i32), ch);
            }
        }
    }
    grid
}

fn sort_antennas(antennas: &HashMap<IVec2, char>) -> HashMap<char, Vec<IVec2>> {
    let mut sorted = HashMap::new();
    for (pos, ch) in antennas {
        sorted.entry(*ch).or_insert_with(Vec::new).push(*pos);
    }
    sorted
}

/// Find the antinodes in the grid.
///
/// Return value is a vector of positions where antinodes are found and the "frequency" of the
/// antinote.
///
/// NOTE:
/// This function now DOES check if the antinode is within the grid.
/// The `x_limit` and `y_limit` parameters are the maximum x and y values of the grid.
/// It is assumed that the grid starts at (0, 0).
fn find_antinodes(
    antennas: &HashMap<char, Vec<IVec2>>,
    (x_limit, y_limit): (i32, i32),
) -> HashSet<(char, IVec2)> {
    let mut antinodes = HashSet::new();

    antennas.iter().for_each(|(ch, positions)| {
        // For each combination of antennas, find the coordinate-difference between them.
        for (i, pos1) in positions.iter().enumerate() {
            for (j, pos2) in positions.iter().enumerate() {
                if i == j {
                    continue;
                }

                // Antinodes are now repeating.
                // Nodes are below each antenna, AND the pattern is repeating out from the antenna.

                // First add the two antinodes below the antennas.
                antinodes.insert((*ch, *pos1));
                antinodes.insert((*ch, *pos2));

                // Now find the difference between the two antennas.
                // This will give us the pattern that repeats out from the antenna.

                // First in one direction.
                let antinode_diff1 = pos1 - pos2;
                let mut start_pos1 = pos1 + antinode_diff1;
                while (0..x_limit).contains(&{ start_pos1.x })
                    && (0..y_limit).contains(&{ start_pos1.y })
                {
                    antinodes.insert((*ch, start_pos1));
                    start_pos1 += antinode_diff1;
                }

                // Now in the other direction.
                let antinode_diff2 = pos2 - pos1;
                let mut start_pos2 = pos2 + antinode_diff2;
                while (0..x_limit).contains(&{ start_pos2.x })
                    && (0..y_limit).contains(&{ start_pos2.y })
                {
                    antinodes.insert((*ch, start_pos2));
                    start_pos2 += antinode_diff2;
                }
            }
        }
    });

    antinodes
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(process(input), 34);
    }

    #[rstest]
    #[case("............", HashMap::new())]
    #[case(
        "........0...", 
        [(IVec2::new(8, 0), '0')].iter().cloned().collect()
    )]
    #[case(
        ".....0......
.......Y....
....A.......", 
        [
            (IVec2::new(5, 0), '0'),
            (IVec2::new(7, 1), 'Y'),
            (IVec2::new(4, 2), 'A'),
        ].iter().cloned().collect()
    )]
    fn test_parse_grid(#[case] input: &str, #[case] expected: HashMap<IVec2, char>) {
        let res = parse_grid_manual(input);
        assert_eq!(res, expected, "Expected: {:#?}, Got: {:#?}", expected, res);
    }

    #[rstest]
    #[case(HashMap::new(), HashMap::new())]
    #[case(
        [
            (IVec2::new(8, 0), '0'),
            (IVec2::new(5, 0), '0'),
            (IVec2::new(7, 1), 'Y'),
            (IVec2::new(4, 2), 'A'),
        ].iter().cloned().collect(),
        [
            ('0', vec![IVec2::new(5, 0), IVec2::new(8, 0)]),
            ('Y', vec![IVec2::new(7, 1)]),
            ('A', vec![IVec2::new(4, 2)]),
        ].iter().cloned().collect()
    )]
    fn test_sort_antennas(
        #[case] antennas: HashMap<IVec2, char>,
        #[case] expected: HashMap<char, Vec<IVec2>>,
    ) {
        let result = sort_antennas(&antennas);

        // Check that all of the results exist in the expected map.
        assert!(
            result.iter().all(|(ch, pos)| {
                expected.contains_key(ch) && pos.iter().all(|p| expected[ch].contains(p))
            }),
            "Expected: {:#?}, Got: {:#?}",
            expected,
            result
        );
    }

    #[rstest]
    #[case(HashMap::new(), HashSet::new(), (5, 5))]
    #[case(HashMap::from([(
            'a',
            vec![IVec2::new(1, 1), IVec2::new(2, 2)]
        )]),
        HashSet::from([
            ('a', IVec2::new(0, 0)),
            ('a', IVec2::new(1, 1)),
            ('a', IVec2::new(2, 2)),
            ('a', IVec2::new(3, 3)),
            ('a', IVec2::new(4, 4)),
        ]),
        (5,5)
    )]
    fn test_find_antinodes(
        #[case] antennas: HashMap<char, Vec<IVec2>>,
        #[case] expected: HashSet<(char, IVec2)>,
        #[case] (x_limit, y_limit): (i32, i32),
    ) {
        let result = find_antinodes(&antennas, (x_limit, y_limit));

        assert_eq!(
            result, expected,
            "Expected: {:#?}, Got: {:#?}",
            expected, result
        );
    }
}

