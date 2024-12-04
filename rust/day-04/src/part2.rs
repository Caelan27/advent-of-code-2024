use std::collections::HashMap;
fn centre_of_cross(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    let neighbors = [
        (-1, -1), // Top-left
        (1, 1),   // Bottom-right
        (-1, 1),  // Top-right
        (1, -1),  // Bottom-left
    ];
    let mut mapped_neighbors: HashMap<(i32, i32), char> = HashMap::new();

    for neighbor in neighbors {
        let new_x = x as i32 + neighbor.0;
        let new_y = y as i32 + neighbor.1;
        if let Some(row) = grid.get(new_y as usize) {
            if let Some(&ch) = row.get(new_x as usize) {
                mapped_neighbors.insert(neighbor, ch);
            }
        }
    }

    if mapped_neighbors.len() != 4 {
        return false;
    } else {
        let &top_left = mapped_neighbors.get(&(-1, -1)).unwrap();
        let &bottom_right = mapped_neighbors.get(&(1, 1)).unwrap();
        let &top_right = mapped_neighbors.get(&(-1, 1)).unwrap();
        let &bottom_left = mapped_neighbors.get(&(1, -1)).unwrap();
        if (bottom_left == 'S' && top_right == 'M' || bottom_left == 'M' && top_right == 'S')
            && (top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M')
        {
            return true;
        }
    }

    false
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut matches = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'A' {
                matches += centre_of_cross(&grid, x, y) as u32;
            }
        }
    }

    matches.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input));
    }
}
