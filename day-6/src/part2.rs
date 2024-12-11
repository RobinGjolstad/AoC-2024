use std::collections::HashSet;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn next(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Step {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    steps: Vec<Step>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Hinderance {
    Obstacle,
    OutOfBounds,
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

fn move_position(pos: Position, direction: Direction) -> Option<Position> {
    match direction {
        Direction::Up => {
            let y = pos.y.checked_sub(1);
            y.map(|y| Position { x: pos.x, y })
        }
        Direction::Down => Some(Position {
            x: pos.x,
            y: pos.y + 1,
        }),
        Direction::Left => {
            let x = pos.x.checked_sub(1);
            x.map(|x| Position { x, y: pos.y })
        }
        Direction::Right => Some(Position {
            x: pos.x + 1,
            y: pos.y,
        }),
    }
}

fn is_within_bounds(pos: Position, grid: &[&str]) -> bool {
    (0..grid.len()).contains(&pos.y) && (0..grid[pos.y].len()).contains(&pos.x)
}

/// Predict the path of the guard starting from the given position and direction
/// until it hits an obstacle or goes out of bounds.
fn predict_guard_path(
    start_position: Position,
    direction: Direction,
    grid: &[&str],
) -> (Path, Hinderance) {
    let mut visited_positions = Path { steps: vec![] };
    let mut current_position = start_position;

    loop {
        if !is_within_bounds(current_position, grid) {
            return (visited_positions, Hinderance::OutOfBounds);
        }

        let current_char = grid[current_position.y]
            .chars()
            .nth(current_position.x)
            .unwrap();
        if current_char == '#' {
            return (visited_positions, Hinderance::Obstacle);
        }

        visited_positions.steps.push(Step {
            position: current_position,
            direction,
        });

        current_position = if let Some(pos) = move_position(current_position, direction) {
            pos
        } else {
            return (visited_positions, Hinderance::OutOfBounds);
        };
    }
}

/// Walk the guard from `pos` in `direction` until an infinite loop is detected or until a maximum
/// of iterations has been exceeded.
fn walk_until_loop(
    pos: Position,
    direction: Direction,
    grid: &[&str],
    max_iterations: usize,
) -> Result<Step, usize> {
    //

    let mut direction = direction;
    let mut current_position = pos;
    let mut visited_positions = Vec::new();
    let mut iterations_remaining = max_iterations;
    loop {
        if iterations_remaining == 0 {
            // We've been walking for too long.
            // println!("Iteration limit exhausted.");
            return Err(visited_positions.len());
        }
        let (path, hinderance) = predict_guard_path(current_position, direction, grid);

        // Check if the guard has visited the same position, in the same direction.
        // If so we have an infinite loop.
        if visited_positions
            .iter()
            .rev()
            .any(|step| path.steps.contains(step))
        {
            // println!(
            //     "Infinite loop detected, breaking. \n Last position: {:?}",
            //     current_position
            // );
            return Ok(*path.steps.last().expect("There should be an element."));
        }

        visited_positions.extend(path.steps);
        if hinderance == Hinderance::OutOfBounds {
            // println!(
            //     "Leaving the grid at {:?}",
            //     visited_positions.last().unwrap().position
            // );
            return Err(visited_positions.len());
        }

        // Update the current position to the last position in the path.
        if let Some(last_step) = visited_positions.last() {
            current_position = last_step.position;
        } else {
            // No more steps.
            return Err(visited_positions.len());
        }

        // Change direction.
        direction = direction.next();
        iterations_remaining -= 1;
    }
}

pub fn process(input: &str) -> usize {
    // Find start position.
    let grid: Vec<&str> = input.lines().collect();
    let start_position = find_start_position(&grid);

    // First walk through the map once.
    // Then we will gradually insert obstacles throughout the path taken, then check if a loop is
    // detected.

    let mut direction = Direction::Up;
    let mut current_position = start_position;
    let mut visited_positions = Vec::new();
    loop {
        let (path, hinderance) = predict_guard_path(current_position, direction, &grid);

        // Check if the guard has visited the same position, in the same direction.
        // If so we have an infinite loop.
        if visited_positions
            .iter()
            .rev()
            .any(|step| path.steps.contains(step))
        {
            println!(
                "Infinite loop detected, breaking. \n Last position: {:?}",
                current_position
            );
            break; // Infinite loop detected.
        }

        visited_positions.extend(path.steps);
        if hinderance == Hinderance::OutOfBounds {
            println!(
                "Leaving the grid at {:?}",
                visited_positions.last().unwrap().position
            );
            break;
        }

        // Update the current position to the last position in the path.
        if let Some(last_step) = visited_positions.last() {
            current_position = last_step.position;
        } else {
            break; // No more steps, break the loop.
        }

        // Change direction.
        direction = direction.next();
    }

    // Now we will gradually insert obstacles for every slot the guard has walked and look for
    // loops.
    let mut obstacle_positions_causing_loop: Vec<Position> = Vec::new();
    for pos in visited_positions {
        // Skip first. No need to insert an obstacle at the start position.
        if pos.position == start_position && pos.direction == Direction::Up {
            continue;
        }

        // Insert an obstacle at the current position.
        let grid_param: Vec<String> = grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                if i == pos.position.y {
                    // This is the row where we want to insert a symbol.
                    row.to_string()
                        .chars()
                        .enumerate()
                        .map(|(i, c)| if i == pos.position.x { '#' } else { c })
                        .collect::<String>()
                } else {
                    row.to_string()
                }
            })
            .collect();

        // Rebuild the grid.
        let grid_param = grid_param
            .iter()
            .map(|line| line.as_str())
            .collect::<Vec<&str>>();

        let loop_res = walk_until_loop(start_position, Direction::Up, &grid_param, 10_000);
        if loop_res.is_ok() && !obstacle_positions_causing_loop.contains(&pos.position) {
            //dbg!("Infinite loop detected with this grid:\n{:#?}", grid_param);
            obstacle_positions_causing_loop.push(pos.position);
        }
    }

    obstacle_positions_causing_loop.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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
        assert_eq!(process(input), 6);
    }

    #[rstest]
    #[case(
        &[".....",
          ".....",
          "..^..",
          "....."],
          Position{x:2, y:2})]
    #[case(
        &[".....",
          ".....",
          ".....",
          "....^"],
          Position{x:4, y:3})]
    #[case(
        &["^....",
          ".....",
          ".....",
          "....."],
          Position{x:0, y:0})]
    fn test_find_start_position(#[case] input: &[&str], #[case] expected: Position) {
        assert_eq!(find_start_position(input), expected);
    }

    #[rstest]
    #[case(6, Hinderance::Obstacle, Direction::Up,
        &["....#.....",
          ".........#",
          "..........",
          "..#.......",
          ".......#..",
          "..........",
          ".#..^.....",
          "........#.",
          "#.........",
          "......#..."])]
    #[case(6, Hinderance::OutOfBounds, Direction::Right,
        &["....#.....",
          ".........#",
          "..........",
          "..#.......",
          ".......#..",
          "..........",
          ".#..^.....",
          "........#.",
          "#.........",
          "......#..."])]
    #[case(3, Hinderance::Obstacle, Direction::Left,
        &["....#.....",
          ".........#",
          "..........",
          "..#.......",
          ".......#..",
          "..........",
          ".#..^.....",
          "........#.",
          "#.........",
          "......#..."])]
    #[case(4, Hinderance::OutOfBounds, Direction::Down,
        &["....#.....",
          ".........#",
          "..........",
          "..#.......",
          ".......#..",
          "..........",
          ".#..^.....",
          "........#.",
          "#.........",
          "......#..."])]
    fn test_predict_guard_path(
        #[case] expected_steps: usize,
        #[case] expected_hinderance: Hinderance,
        #[case] direction: Direction,
        #[case] grid: &[&str],
    ) {
        let start_position = find_start_position(grid);
        let (path, hinderance) = predict_guard_path(start_position, direction, grid);
        assert_eq!(path.steps.len(), expected_steps);
        assert_eq!(hinderance, expected_hinderance);
    }

    #[rstest]
    #[case(Position { x: 2, y: 2 }, Direction::Up, Some(Position{x:2,y:1}))]
    #[case(Position { x: 2, y: 2 }, Direction::Down, Some(Position{x:2,y:3}))]
    #[case(Position { x: 2, y: 2 }, Direction::Left, Some(Position{x:1,y:2}))]
    #[case(Position { x: 2, y: 2 }, Direction::Right, Some(Position{x:3,y:2}))]
    #[case(Position { x: 0, y: 0 }, Direction::Up, None)] // Boundary case
    #[case(Position { x: 0, y: 0 }, Direction::Left, None)] // Boundary case
    fn test_move_position(
        #[case] start_position: Position,
        #[case] direction: Direction,
        #[case] expected_position: Option<Position>,
    ) {
        let new_position = move_position(start_position, direction);
        assert_eq!(new_position, expected_position);
    }

    #[rstest]
    #[case(Position { x: 2, y: 2 }, &[".....", ".....", "....."], true)]
    #[case(Position { x: 4, y: 2 }, &[".....", ".....", "....."], true)]
    #[case(Position { x: 5, y: 2 }, &[".....", ".....", "....."], false)] // Out of bounds
    #[case(Position { x: 2, y: 3 }, &[".....", ".....", "....."], false)] // Out of bounds
    #[case(Position { x: 0, y: 0 }, &[".....", ".....", "....."], true)]
    #[case(Position { x: 0, y: 0 }, &[], false)] // Empty grid
    fn test_is_within_bounds(
        #[case] position: Position,
        #[case] grid: &[&str],
        #[case] expected: bool,
    ) {
        let result = is_within_bounds(position, grid);
        assert_eq!(result, expected);
    }

    /*
        test-grid
    r#"....#.....
    .........#
    ..........
    ..#.......
    .......#..
    ..........
    .#..^.....
    ........#.
    #.........
    ......#..."#;
        */
    #[rstest]
    #[case(
        Position { x: 4, y: 6 },
        Direction::Up,
        Position { x: 3, y: 6 },
        &["....#.....",
          ".........#",
          "..........",
          "..#.......",
          ".......#..",
          "..........",
          ".#..^.....",
          "........#.",
          "#.........",
          "......#..."])]
    fn test_added_obstacle_causes_loop(
        #[case] guard_position: Position,
        #[case] guard_direction: Direction,
        #[case] obstacle_position: Position,
        #[case] grid: &[&str],
    ) {
        // Add an obstacle to the grid.
        let mut grid = grid
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>();
        grid[obstacle_position.y] = grid[obstacle_position.y]
            .chars()
            .enumerate()
            .map(|(i, c)| if i == obstacle_position.x { '#' } else { c })
            .collect::<String>();

        // Collect to the original format to conform with the function signature.
        let grid = grid.iter().map(|line| line.as_str()).collect::<Vec<&str>>();

        let inf_res = walk_until_loop(guard_position, guard_direction, &grid, 100);
        assert!(
            inf_res.is_ok(),
            "We should have encountered an infinite loop, but the result was {:?}",
            inf_res
        );
    }
}
