use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum PositionType {
    Guard(Direction),
    Obstruction,
    Empty,
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn move_guard(
    grid: &mut [Vec<PositionType>],
    (initial_guard_x, initial_guard_y): (usize, usize),
    unique_positions: &mut HashSet<(u32, u32)>,
) -> Option<(usize, usize)> {
    // Copies of the initial guard position so we can mutate them
    let mut guard_x = initial_guard_x;
    let mut guard_y = initial_guard_y;

    // The guard's type (up, down, left, right)
    let guard = &grid[initial_guard_y][initial_guard_x];

    // The number of rows and columns in the grid
    let (rows, columns) = (grid.len(), grid[0].len());

    // Move the guard in the direction it's facing until it either hits an obstruction or the edge
    // of the grid
    match &guard {
        PositionType::Guard(Direction::Up) => {
            while guard_y != 0 && grid[guard_y - 1][guard_x] != PositionType::Obstruction {
                guard_y -= 1;
                unique_positions.insert((guard_x as u32, guard_y as u32));
            }
        }
        PositionType::Guard(Direction::Right) => {
            while guard_x != columns - 1 && grid[guard_y][guard_x + 1] != PositionType::Obstruction
            {
                guard_x += 1;
                unique_positions.insert((guard_x as u32, guard_y as u32));
            }
        }
        PositionType::Guard(Direction::Down) => {
            while guard_y != rows - 1 && grid[guard_y + 1][guard_x] != PositionType::Obstruction {
                guard_y += 1;
                unique_positions.insert((guard_x as u32, guard_y as u32));
            }
        }
        PositionType::Guard(Direction::Left) => {
            while guard_x != 0 && grid[guard_y][guard_x - 1] != PositionType::Obstruction {
                guard_x -= 1;
                unique_positions.insert((guard_x as u32, guard_y as u32));
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

    // If the guard hits the end of the grid, it returns None to signal the end of the simulation
    if guard_x == 0 || guard_x == columns - 1 || guard_y == 0 || guard_y == rows - 1 {
        return None;
    }

    // Reset the initial guard position to empty
    grid[initial_guard_y][initial_guard_x] = PositionType::Empty;

    // Return the new guard position
    Some((guard_x, guard_y))
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    // Parses the grid
    let mut grid: Vec<Vec<PositionType>> = input
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
    let (mut guard_x, mut guard_y) = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if let PositionType::Guard(_direction) = char {
                guard_x = x;
                guard_y = y;
                break;
            }
        }
    }

    // A set of unique positions the guard has visited
    let mut unique_positions = HashSet::new();
    unique_positions.insert((guard_x as u32, guard_y as u32));

    // Moves the guard until it hits the edge of the grid
    loop {
        (guard_x, guard_y) = match move_guard(&mut grid, (guard_x, guard_y), &mut unique_positions)
        {
            Some((x, y)) => (x, y),
            None => break,
        };
    }

    // Returns the number of unique positions the guard has visited
    unique_positions.len().to_string()
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
        assert_eq!("41", process(input));
    }
}
