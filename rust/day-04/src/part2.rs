fn centre_of_cross(grid: &[Vec<char>], x: usize, y: usize) -> bool {
    if y > 0 && y < grid.len() - 1 && x > 0 && x < grid[y].len() - 1 {
        let &top_left = grid.get(y - 1).and_then(|row| row.get(x - 1)).unwrap();
        let &top_right = grid.get(y - 1).and_then(|row| row.get(x + 1)).unwrap();
        let &bottom_left = grid.get(y + 1).and_then(|row| row.get(x - 1)).unwrap();
        let &bottom_right = grid.get(y + 1).and_then(|row| row.get(x + 1)).unwrap();
        (bottom_left == 'S' && top_right == 'M' || bottom_left == 'M' && top_right == 'S')
            && (top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M')
    } else {
        false
    }
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
