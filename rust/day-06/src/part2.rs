use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum PositionType {
    Guard(Direction),
    Obstruction,
    Empty,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum MoveGuardResult {
    Moved((usize, usize)),
    Ended,
    Looped,
}

fn num_loops(
    grid: &[Vec<PositionType>],
    (initial_guard_x, initial_guard_y): (usize, usize),
) -> u32 {
    // Makes a copy of the grid so we can mutate it
    let mut temp_grid = grid.to_owned();

    // Copies of the initial guard position so we can mutate them
    let mut guard_x = initial_guard_x;
    let mut guard_y = initial_guard_y;

    // Run a simulation to show all positions the guard will visit
    let mut guard_positions = HashSet::new();
    loop {
        (guard_x, guard_y) =
            match move_guard(&mut temp_grid, (guard_x, guard_y), &mut guard_positions) {
                MoveGuardResult::Moved((x, y)) => (x, y),
                MoveGuardResult::Ended => {
                    break;
                }
                MoveGuardResult::Looped => {
                    break;
                }
            };
    }

    // Initialises a counter to count the number of loops
    let mut loop_count = 0;

    // Takes all visited positions that aren't the start position
    let mut visited_positions = HashSet::new();
    for (x, y, _direction) in guard_positions.into_iter() {
        if (x, y) != (initial_guard_x, initial_guard_y) {
            visited_positions.insert((x, y));
        }
    }

    for (x, y) in visited_positions.into_iter() {
        // Copies of the initial guard position so we can mutate them
        let (mut guard_x, mut guard_y) = (initial_guard_x, initial_guard_y);
        // Copies the grid so we can mutate it
        let mut temp_grid = grid.to_owned();

        // Sets the guard's positions so we can see if it's looped
        let mut temp_guard_positions = HashSet::new();
        temp_guard_positions.insert((initial_guard_x, initial_guard_y, Direction::Up));

        // Places an obstruction at the visited position
        temp_grid[y][x] = PositionType::Obstruction;

        // Run a simulation to see if the guard loops
        loop {
            (guard_x, guard_y) = match move_guard(
                &mut temp_grid,
                (guard_x, guard_y),
                &mut temp_guard_positions,
            ) {
                MoveGuardResult::Moved((x, y)) => (x, y),
                MoveGuardResult::Ended => {
                    break;
                }
                MoveGuardResult::Looped => {
                    loop_count += 1;
                    break;
                }
            };
        }
    }

    // Returns the number of loops
    loop_count
}

fn move_guard(
    grid: &mut [Vec<PositionType>],
    (initial_guard_x, initial_guard_y): (usize, usize),
    guard_positions: &mut HashSet<(usize, usize, Direction)>,
) -> MoveGuardResult {
    // Copies of the initial guard position so we can mutate them
    let mut guard_x = initial_guard_x;
    let mut guard_y = initial_guard_y;

    // The guard's type (up, down, left, right)
    let guard = &grid[initial_guard_y][initial_guard_x];

    // The number of rows and columns in the grid
    let (rows, columns) = (grid.len(), grid[0].len());

    // Moves the guard in the direction it's facing until it either hits an obstruction or the edge
    match &guard {
        PositionType::Guard(Direction::Up) => {
            while guard_y != 0 && grid[guard_y - 1][guard_x] != PositionType::Obstruction {
                guard_y -= 1;
                // If it's been in the same position before, return that it's looped
                if guard_positions.contains(&(guard_x, guard_y, Direction::Up)) {
                    return MoveGuardResult::Looped;
                }
                guard_positions.insert((guard_x, guard_y, Direction::Up));
            }
        }

        PositionType::Guard(Direction::Right) => {
            while guard_x != columns - 1 && grid[guard_y][guard_x + 1] != PositionType::Obstruction
            {
                guard_x += 1;
                if guard_positions.contains(&(guard_x, guard_y, Direction::Right)) {
                    return MoveGuardResult::Looped;
                }
                guard_positions.insert((guard_x, guard_y, Direction::Right));
            }
        }

        PositionType::Guard(Direction::Down) => {
            while guard_y != rows - 1 && grid[guard_y + 1][guard_x] != PositionType::Obstruction {
                guard_y += 1;
                if guard_positions.contains(&(guard_x, guard_y, Direction::Down)) {
                    return MoveGuardResult::Looped;
                }
                guard_positions.insert((guard_x, guard_y, Direction::Down));
            }
        }

        PositionType::Guard(Direction::Left) => {
            while guard_x != 0 && grid[guard_y][guard_x - 1] != PositionType::Obstruction {
                guard_x -= 1;
                if guard_positions.contains(&(guard_x, guard_y, Direction::Left)) {
                    return MoveGuardResult::Looped;
                }
                guard_positions.insert((guard_x, guard_y, Direction::Left));
            }
        }
        _ => panic!("unexpected position type"),
    }

    // Change the guard's direction once it hits an obstruction or the edge of the grid
    grid[guard_y][guard_x] = match guard {
        PositionType::Guard(Direction::Up) => PositionType::Guard(Direction::Right),
        PositionType::Guard(Direction::Right) => PositionType::Guard(Direction::Down),
        PositionType::Guard(Direction::Down) => PositionType::Guard(Direction::Left),
        PositionType::Guard(Direction::Left) => PositionType::Guard(Direction::Up),
        _ => panic!("unexpected position type"),
    };

    // Return ended if the guard hits the end of the grid
    if guard_x == 0 || guard_x == columns - 1 || guard_y == 0 || guard_y == rows - 1 {
        return MoveGuardResult::Ended;
    }

    // Reset the initial guard position to empty as long as the guard hasn't stayed there
    if (initial_guard_x, initial_guard_y) != (guard_x, guard_y) {
        grid[initial_guard_y][initial_guard_x] = PositionType::Empty;
    }

    // Return the new guard position
    MoveGuardResult::Moved((guard_x, guard_y))
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    // Parses the grid
    let grid: Vec<Vec<PositionType>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => PositionType::Empty,
                    '^' => PositionType::Guard(Direction::Up),
                    '>' => PositionType::Guard(Direction::Right),
                    '<' => PositionType::Guard(Direction::Left),
                    'v' => PositionType::Guard(Direction::Down),
                    '#' => PositionType::Obstruction,
                    _ => panic!("unexpected character"),
                })
                .collect()
        })
        .collect();

    // Sets the initial guard position
    let mut initial_guard_pos = (0, 0);

    // Finds the initial guard position
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let PositionType::Guard(_) = cell {
                initial_guard_pos = (x, y);
                break;
            }
        }
    }

    // Finds the number of loops the guard will make
    let num_loops = num_loops(&grid, initial_guard_pos);

    // Returns the number of loops as a string
    num_loops.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("6", process(input));
    }
}
