use std::collections::HashSet;

use glam::UVec2;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Position {
    position: UVec2,
    height: u32,
}

fn parse_mountain(input: &str) -> Vec<Vec<Position>> {
    let mut mountain = vec![];
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut mountain_line = vec![];
        for (x, c) in line.chars().enumerate() {
            let height = c.to_digit(10).map_or_else(|| 0x45, |d| d);

            mountain_line.push(Position {
                position: UVec2::new(x as u32, y as u32),
                height,
            });
        }
        mountain.push(mountain_line);
    }
    mountain
}

fn find_trailheads(mountain: &Vec<Vec<Position>>) -> Vec<Position> {
    mountain
        .iter()
        .enumerate()
        .flat_map(|(mount_idx, mount_line)| {
            mount_line.iter().enumerate().filter_map(|(pos_idx, pos)| {
                if pos.height == 0 {
                    Some(*pos)
                } else {
                    None
                }
            })
        })
        .collect()
}

fn traverse_mountain(
    current_position: &Position,
    path_taken: &[Position],
    mountain: &Vec<Vec<Position>>,
) -> Vec<Vec<Position>> {
    // Find positions around "self" where the height is one higher.
    // For each of these, start a new recursive traversal from that position, but make sure to
    // append the current function to "path_taken" before passing it in.
    // If one of the paths leads to a 9, add it to the path and return that as a solution.

    let mountain_dimensions = (
        mountain
            .first()
            .expect("We should never have empty lists")
            .len(),
        mountain.len(),
    );
    let one_step_up = current_position.height + 1;

    // Get coordinates for direction.
    let up_coords = match (
        current_position.position.x,
        current_position.position.y.checked_sub(1),
    ) {
        (x, Some(y)) => Some((x, y)),
        _ => None,
    };
    let right_coords =
        if (0..mountain_dimensions.0).contains(&((current_position.position.x + 1) as usize)) {
            Some((current_position.position.x + 1, current_position.position.y))
        } else {
            None
        };
    let down_coords =
        if (0..mountain_dimensions.1).contains(&((current_position.position.y + 1) as usize)) {
            Some((current_position.position.x, current_position.position.y + 1))
        } else {
            None
        };
    let left_coords = match (
        current_position.position.x.checked_sub(1),
        current_position.position.y,
    ) {
        (Some(x), y) => Some((x, y)),
        _ => None,
    };

    // Now check each direction if it contains a slot with a height of H+1 or 9.
    // If so, either start another run or return a list of the current path.
    let up_lists = if let Some((x, y)) = up_coords {
        let next_position = mountain[y as usize][x as usize];
        let next_height = next_position.height;

        let mut path_taken = path_taken.to_vec();
        path_taken.push(*current_position);

        if next_height == 9 && one_step_up == 9 {
            path_taken.push(next_position);
            // println!("Moving up: Found path to top: {:?}", path_taken);
            vec![path_taken]
        } else if next_height == one_step_up {
            traverse_mountain(&next_position, &path_taken, mountain)
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let right_lists = if let Some((x, y)) = right_coords {
        let next_position = mountain[y as usize][x as usize];
        let next_height = next_position.height;

        let mut path_taken = path_taken.to_vec();
        path_taken.push(*current_position);

        if next_height == 9 && one_step_up == 9 {
            path_taken.push(next_position);
            // println!("Moving right: Found path to top: {:?}", path_taken);
            vec![path_taken]
        } else if next_height == one_step_up {
            traverse_mountain(&next_position, &path_taken, mountain)
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let down_lists = if let Some((x, y)) = down_coords {
        let next_position = mountain[y as usize][x as usize];
        let next_height = next_position.height;

        let mut path_taken = path_taken.to_vec();
        path_taken.push(*current_position);

        if next_height == 9 && one_step_up == 9 {
            path_taken.push(next_position);
            // println!("Moving down: Found path to top: {:?}", path_taken);
            vec![path_taken]
        } else if next_height == one_step_up {
            traverse_mountain(&next_position, &path_taken, mountain)
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let left_lists = if let Some((x, y)) = left_coords {
        let next_position = mountain[y as usize][x as usize];
        let next_height = next_position.height;

        let mut path_taken = path_taken.to_vec();
        path_taken.push(*current_position);

        if next_height == 9 && one_step_up == 9 {
            path_taken.push(next_position);
            // println!("Moving left: Found path to top: {:?}", path_taken);
            vec![path_taken]
        } else if next_height == one_step_up {
            traverse_mountain(&next_position, &path_taken, mountain)
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    let mut return_vec = vec![];
    return_vec.extend(up_lists);
    return_vec.extend(right_lists);
    return_vec.extend(down_lists);
    return_vec.extend(left_lists);

    return_vec
}

pub fn process(input: &str) -> usize {
    let mountain = parse_mountain(input);
    let trailheads = find_trailheads(&mountain);

    let paths: Vec<Vec<Position>> = trailheads
        .into_iter()
        .enumerate()
        .flat_map(|(idx, trailhead)| traverse_mountain(&trailhead, &[], &mountain))
        .collect();

    // Count all possible paths.
    paths.len()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(
        ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
        3
    )]
    #[case(
        "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        13
    )]
    #[case(
        "012345
123456
234567
345678
4.6789
56789.",
        227
    )]
    #[case(
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        81
    )]
    fn test_process(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(process(input), expected);
    }
}

