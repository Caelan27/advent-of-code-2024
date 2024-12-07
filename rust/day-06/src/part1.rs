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
    let mut guard_x = initial_guard_x;
    let mut guard_y = initial_guard_y;

    let guard = &grid[initial_guard_y][initial_guard_x];

    let (rows, columns) = (grid.len(), grid[0].len());

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

    grid[guard_y][guard_x] = match guard {
        PositionType::Guard(Direction::Up) => PositionType::Guard(Direction::Right),
        PositionType::Guard(Direction::Right) => PositionType::Guard(Direction::Down),
        PositionType::Guard(Direction::Down) => PositionType::Guard(Direction::Left),
        PositionType::Guard(Direction::Left) => PositionType::Guard(Direction::Up),
        _ => panic!("unexpected position type"),
    };

    if guard_x == 0 || guard_x == columns - 1 || guard_y == 0 || guard_y == rows - 1 {
        return None;
    }

    grid[initial_guard_y][initial_guard_x] = PositionType::Empty;

    Some((guard_x, guard_y))
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
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

    let mut unique_positions = HashSet::new();
    unique_positions.insert((guard_x as u32, guard_y as u32));

    loop {
        (guard_x, guard_y) = match move_guard(&mut grid, (guard_x, guard_y), &mut unique_positions)
        {
            Some((x, y)) => (x, y),
            None => break,
        };
    }

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
