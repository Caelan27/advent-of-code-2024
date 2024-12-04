fn is_start_of_word(grid: &[Vec<char>], x: usize, y: usize) -> u32 {
    let neighbors = [
        (-1, 0),  // Up
        (0, -1),  // Left
        (1, 0),   // Down
        (-1, -1), // Top-left
        (-1, 1),  // Top-right
        (1, -1),  // Bottom-left
        (1, 1),   // Bottom-right
        (0, 1),   // Right
    ];

    let target_word = ['X', 'M', 'A', 'S'];

    let check_direction = |dx: i32, dy: i32| -> bool {
        target_word.iter().enumerate().all(|(i, &ch)| {
            let new_x = x as i32 + i as i32 * dx;
            let new_y = y as i32 + i as i32 * dy;

            grid.get(new_y as usize)
                .and_then(|row| row.get(new_x as usize))
                .map_or(false, |&grid_ch| grid_ch == ch)
        })
    };

    neighbors
        .iter()
        .filter(|&(dx, dy)| check_direction(*dx, *dy))
        .count() as u32
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut matches = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'X' {
                matches += is_start_of_word(&grid, x, y);
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
        assert_eq!("18", process(input));
    }
}
