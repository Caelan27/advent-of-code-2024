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
    grid: &Vec<Vec<PositionType>>,
    (initial_guard_x, initial_guard_y): (usize, usize),
) -> u32 {
    let mut temp_grid = grid.clone();
    let mut guard_positions = HashSet::new();

    let mut guard_x = initial_guard_x;
    let mut guard_y = initial_guard_y;

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

    let mut loop_count = 0;
    let mut visited_positions = HashSet::new();
    for (x, y, _direction) in guard_positions.into_iter() {
        visited_positions.insert((x, y));
    }
    for (x, y) in visited_positions.into_iter() {
        if (x, y) == (initial_guard_x, initial_guard_y) {
            continue;
        }
        let (mut guard_x, mut guard_y) = (initial_guard_x, initial_guard_y);
        let mut temp_grid = grid.clone();

        let mut temp_guard_positions = HashSet::new();
        temp_guard_positions.insert((initial_guard_x, initial_guard_y, Direction::Up));

        temp_grid[y][x] = PositionType::Obstruction;

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

    loop_count
}

fn move_guard(
    grid: &mut [Vec<PositionType>],
    (initial_guard_x, initial_guard_y): (usize, usize),
    guard_positions: &mut HashSet<(usize, usize, Direction)>,
) -> MoveGuardResult {
    let mut guard_x = initial_guard_x;
    let mut guard_y = initial_guard_y;

    let guard = &grid[initial_guard_y][initial_guard_x];

    let (rows, columns) = (grid.len(), grid[0].len());

    match &guard {
        PositionType::Guard(Direction::Up) => {
            while guard_y != 0 && grid[guard_y - 1][guard_x] != PositionType::Obstruction {
                guard_y -= 1;
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

    grid[guard_y][guard_x] = match guard {
        PositionType::Guard(Direction::Up) => PositionType::Guard(Direction::Right),
        PositionType::Guard(Direction::Right) => PositionType::Guard(Direction::Down),
        PositionType::Guard(Direction::Down) => PositionType::Guard(Direction::Left),
        PositionType::Guard(Direction::Left) => PositionType::Guard(Direction::Up),
        _ => panic!("unexpected position type"),
    };

    if guard_x == 0 || guard_x == columns - 1 || guard_y == 0 || guard_y == rows - 1 {
        return MoveGuardResult::Ended;
    }

    if (initial_guard_x, initial_guard_y) != (guard_x, guard_y) {
        grid[initial_guard_y][initial_guard_x] = PositionType::Empty;
    }

    MoveGuardResult::Moved((guard_x, guard_y))
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
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

    let mut initial_guard_pos = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if let PositionType::Guard(_) = cell {
                initial_guard_pos = (x, y);
                break;
            }
        }
    }

    let num_loops = num_loops(&grid, initial_guard_pos);

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
