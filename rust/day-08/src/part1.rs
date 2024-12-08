use std::collections::HashMap;
use std::collections::HashSet;

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut antenna_positions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character != '.' {
                antenna_positions.entry(character).or_default().push((x, y));
            }
        }
    }

    let lines = input.lines().collect::<Vec<_>>().len();
    let columns = input.lines().next().unwrap().len();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (_character, positions) in antenna_positions.iter() {
        for (index, (x1, y1)) in positions.iter().enumerate() {
            for (x2, y2) in positions.iter().skip(index + 1) {
                let dx = *x2 as isize - *x1 as isize;
                let dy = *y2 as isize - *y1 as isize;

                let antinode_x1 = *x1 as isize - dx;
                let antinode_y1 = *y1 as isize - dy;

                let antinode_x2 = *x2 as isize + dx;
                let antinode_y2 = *y2 as isize + dy;

                if antinode_x1 >= 0
                    && antinode_x1 < columns as isize
                    && antinode_y1 >= 0
                    && antinode_y1 < lines as isize
                {
                    antinodes.insert((antinode_x1 as usize, antinode_y1 as usize));
                }

                if antinode_x2 >= 0
                    && antinode_x2 < columns as isize
                    && antinode_y2 >= 0
                    && antinode_y2 < lines as isize
                {
                    antinodes.insert((antinode_x2 as usize, antinode_y2 as usize));
                }
            }
        }
    }

    antinodes.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input));
    }
}
