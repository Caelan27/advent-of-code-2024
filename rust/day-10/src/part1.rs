use std::collections::HashSet;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn find_next(x: usize, y: usize, map: &[Vec<u8>]) -> Option<Vec<(usize, usize)>> {
    let looking_for = map[y][x] + 1;
    let neighbors_relative: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut neighbors = Vec::new();
    let rows = map.len();
    let cols = map[0].len();

    for (neighbor_x, neighbor_y) in neighbors_relative {
        if (neighbor_x + x as isize) < 0
            || (neighbor_y + y as isize) < 0
            || (neighbor_x + x as isize) >= cols as isize
            || (neighbor_y + y as isize) >= rows as isize
        {
            continue;
        } else {
            let (abs_x, abs_y) = (x as isize, y as isize);
            neighbors.push(((neighbor_x + abs_x) as usize, (neighbor_y + abs_y) as usize));
        }
    }

    let mut vec = Vec::new();
    for neighbor in neighbors {
        if map[neighbor.1][neighbor.0] == looking_for {
            vec.push(neighbor);
        }
    }

    if vec.is_empty() {
        None
    } else {
        Some(vec)
    }
}

fn trailhead_count(map: &[Vec<u8>], x: usize, y: usize) -> usize {
    let mut current_positions = HashSet::new();
    current_positions.insert((x, y));
    let mut current_digit = 0;

    // Repeat until broken - When the current position is 9 or there are no more positions to check
    loop {
        // Increment the current digit
        current_digit += 1;

        // Create a vec to store the positions for the next iteration
        let mut next_positions = Vec::new();

        // For each position in the current positions
        for position in current_positions.iter() {
            // If there is a next position, add it to the next positions
            if let Some(next) = find_next(position.0, position.1, map) {
                next_positions.extend(next);
            }
        }

        current_positions.clear();

        // If there are no next positions, return 0
        if next_positions.is_empty() {
            return 0;
        } else {
            // Otherwise, set the current positions to the found positions
            for position in next_positions {
                current_positions.insert(position);
            }
        }

        if current_digit == 9 {
            return current_positions.len();
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let map = parse(input);

    let mut total = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, digit) in row.iter().enumerate() {
            if *digit == 0 {
                total += trailhead_count(&map, x, y);
            }
        }
    }
    dbg!(&total);

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input));
    }
}
