use std::collections::HashSet;

pub fn process(input: &str) -> usize {
    // Find start position.
    let grid: Vec<&str> = input.lines().collect();
    let start_position = find_start_position(&grid);

    // Find the first obstacle moving up.
    // Keep the position _before_ the obstacle.
    // Collect all positions from start to, but not including, the obstacle.
    // Rotate right, and repeat the process.
    // Keep going until we're heading out of bounds.
    let mut current_position = start_position;
    let mut direction = Direction::Up;
    let mut obstacle = find_obstacle(current_position, direction, &grid);
    let mut positions = vec![];

    while let Some(obstacle_pos) = obstacle {
        let new_positions = collect_positions(&current_position, &obstacle_pos);
        println!(
            "New positions in direction {:?}: {:#?}\n\tObstacle at {:?}",
            direction, new_positions, obstacle
        );
        positions.extend(new_positions);
        current_position = positions.last().copied().unwrap();
        direction = direction.next().unwrap(); // Rotate right.
        obstacle = find_obstacle(current_position, direction, &grid);
    }

    // Now we're heading out of bounds.
    // Find the exit position.
    // Collect all positions from the current position to, but not including, the exit position.
    let exit_position = find_exit_position(current_position, direction, &grid);
    let new_positions = collect_positions(&current_position, &exit_position);
    println!(
        "New positions in direction {:?}: {:#?}\nExit at {:?}",
        direction, new_positions, exit_position
    );
    positions.extend(new_positions);

    // Calculate the sum of the positions.
    // Exclude duplicates.
    let final_pos = positions.iter().collect::<HashSet<_>>();
    println!("Final positions: {:#?}", final_pos);

    final_pos.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Iterator for Direction {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Up => {
                *self = Self::Right;
                Some(Self::Right)
            }
            Self::Right => {
                *self = Self::Down;
                Some(Self::Down)
            }
            Self::Down => {
                *self = Self::Left;
                Some(Self::Left)
            }
            Self::Left => {
                *self = Self::Up;
                Some(Self::Up)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

fn find_start_position(grid: &[&str]) -> Position {
    let mut x = 0;
    let mut y = 0;
    for (i, line) in grid.iter().enumerate() {
        if let Some(j) = line.find('^') {
            x = j;
            y = i;
            break;
        }
    }
    (x, y).into()
}

fn find_obstacle(
    start_position: Position,
    direction: Direction,
    grid: &[&str],
) -> Option<Position> {
    match direction {
        Direction::Right => {
            // Scan grid from the start position to the right until a `#` is found.
            let found_pos = grid[start_position.y]
                .chars()
                .enumerate()
                .skip(start_position.x)
                .find(|&(_, ch)| ch == '#')
                .map(|(idx, _)| idx);

            found_pos.map(|x| Position {
                x,
                y: start_position.y,
            })
        }
        Direction::Left => {
            // Scan grid from the start position to the left until a `#` is found.
            let found_pos = (0..start_position.x)
                .rev()
                .find(|&idx| grid[start_position.y].chars().nth(idx) == Some('#'));

            found_pos.map(|x| Position {
                x,
                y: start_position.y,
            })
        }
        Direction::Down => {
            // Scan grid from the start position downwards until a `#` is found.
            let found_pos = grid
                .iter()
                .enumerate()
                .skip(start_position.y)
                .find(|&(_, line)| line.chars().nth(start_position.x) == Some('#'))
                .map(|(idx, _)| idx);

            found_pos.map(|y| Position {
                x: start_position.x,
                y,
            })
        }
        Direction::Up => {
            // Scan grid from the start position upwards until a `#` is found.
            let found_pos = (0..start_position.y)
                .rev()
                .find(|&idx| grid[idx].chars().nth(start_position.x) == Some('#'));

            found_pos.map(|y| Position {
                x: start_position.x,
                y,
            })
        }
    }
}

/// Helper to find the point of "exit" from the grid.
///
/// We don't care about obstacles here, we just want to find the point where we're heading out of bounds.
fn find_exit_position(current_position: Position, direction: Direction, grid: &[&str]) -> Position {
    match direction {
        Direction::Right => Position {
            x: grid[current_position.y].len(),
            y: current_position.y,
        },
        Direction::Left => Position {
            x: 0,
            y: current_position.y,
        },
        Direction::Down => Position {
            x: current_position.x,
            y: grid.len(),
        },
        Direction::Up => Position {
            x: current_position.x,
            y: 0,
        },
    }
}

/// Collects all positions from `from` to, but not including, `to`.
///
/// #Panics
/// This function will panic if `from` and `to` are not aligned in one of the axes.
fn collect_positions(from: &Position, to: &Position) -> Vec<Position> {
    let x_aligned = (from.x == to.x);
    let y_aligned = (from.y == to.y);
    assert!(x_aligned || y_aligned, "Axes must be aligned.");

    // Determine which direction to scan.
    // TODO: Deduplicate this code when we feel like cleaning up.
    if x_aligned && y_aligned {
        // from and to are at the same position.
        vec![*from]
    } else if x_aligned {
        // Find all cells in x-direction.
        // Determine the direction.
        // We might need to reverse the range.
        let (from_range, to_range, reverse) = if from.y < to.y {
            (from, to, false)
        } else {
            (to, from, true)
        };

        // Collect all positions.
        let res: Vec<Position> = (from_range.y..=to_range.y)
            .map(|y| Position { x: from_range.x, y })
            .collect();

        // Remove "to" from the result.
        let mut res: Vec<Position> = res.into_iter().filter(|pos| pos != to).collect();

        // Reverse the result if needed.
        if reverse {
            res.reverse();
        }
        res
    } else {
        // Find all cells in y-direction.
        // Determine the direction.
        // We might need to reverse the range.
        let (from_range, to_range, reverse) = if from.x < to.x {
            (from, to, false)
        } else {
            (to, from, true)
        };

        // Collect all positions.
        let res: Vec<Position> = (from_range.x..=to_range.x)
            .map(|x| Position { x, y: from_range.y })
            .collect();

        // Remove "to" from the result.
        let mut res: Vec<Position> = res.into_iter().filter(|pos| pos != to).collect();

        // Reverse the result if needed.
        if reverse {
            res.reverse();
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_process() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(process(input), 41);
    }

    #[rstest]
    #[case(&["....^....."], (4, 0))]
    #[case(
        &[".....",
          ".....",
          "..^..",
          ".....",], (2,2))]
    fn test_find_start_position(#[case] input: &[&str], #[case] expected: (usize, usize)) {
        assert_eq!(find_start_position(input), expected.into());
    }

    #[rstest]
    #[case((0,0), Direction::Right, &["...#"], (3,0))]
    #[case((0,0), Direction::Right, &["...#....#"], (3,0))]
    #[case((0,0), Direction::Right, &["?a*#....#"], (3,0))]
    #[case((2,0), Direction::Right, &["#.......#"], (8,0))]
    #[case((5,0), Direction::Left, &["#....."], (0,0))]
    #[case((5,0), Direction::Left, &["#.?+%."], (0,0))]
    #[case((10,0), Direction::Left, &["#.?+%.#......."], (6,0))]
    #[case((2,0), Direction::Left, &["#.?+%.#......."], (0,0))]
    #[case((0,0), Direction::Down, &[".", ".","#"], (0, 2))]
    #[case((0,1), Direction::Down, &["#",".",".",".",".","#"], (0, 5))]
    #[case((0,1), Direction::Down, &[".","?","%","#"], (0, 3))]
    #[case((0,3), Direction::Up, &["#",".",".","."], (0,0))]
    #[case((0,3), Direction::Up, &["#","?","%","."], (0,0))]
    #[case((0,4), Direction::Up, &["#",".",".",".",".","#"], (0, 0))]
    fn test_find_obstacle(
        #[case] start_position: (usize, usize),
        #[case] direction: Direction,
        #[case] grid: &[&str],
        #[case] expected: (usize, usize),
    ) {
        let res = find_obstacle(start_position.into(), direction, grid);
        assert!(
            res.is_some_and(|pos| pos == expected.into()),
            "Direction {:?} with input: {:#?}:\n\tExpected to find obstacle at: {:?}. Result was: {:?}",
            direction,
            grid,
            expected,
            res
        );
    }

    #[rstest]
    #[case((0,0), (0,0), vec![(0,0)])]
    #[case((0,0), (0,3), vec![(0,0), (0,1), (0,2)])]
    #[case((0,0), (3,0), vec![(0,0), (1,0), (2,0)])]
    #[case((3,0), (0,0), vec![(3,0), (2,0), (1,0)])]
    fn test_collect_positions(
        #[case] from: (usize, usize),
        #[case] to: (usize, usize),
        #[case] expected: Vec<(usize, usize)>,
    ) {
        let res = collect_positions(&from.into(), &to.into());
        assert_eq!(
            res,
            expected
                .into_iter()
                .map(|pos| pos.into())
                .collect::<Vec<_>>()
        );
    }
}
