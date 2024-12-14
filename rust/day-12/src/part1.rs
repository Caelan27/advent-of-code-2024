use std::collections::HashMap;
use std::collections::HashSet;

fn calculate_price(region: &HashSet<(usize, usize)>) -> usize {
    let area = region.len();
    let perimeter = region.iter().fold(0, |acc: isize, (x, y)| {
        let mut perimeter = 4;
        if region.contains(&(*x + 1, *y)) {
            perimeter -= 1;
        }
        if *x > 0 && region.contains(&(*x - 1, *y)) {
            perimeter -= 1;
        }
        if region.contains(&(*x, *y + 1)) {
            perimeter -= 1;
        }
        if *y > 0 && region.contains(&(*x, *y - 1)) {
            perimeter -= 1;
        }
        acc + perimeter
    });
    area * perimeter as usize
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut plot_types: HashSet<char> = HashSet::new();
    let mut neighbors: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut in_region: HashSet<(usize, usize)> = HashSet::new();
    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();

    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .inspect(|ch| {
                    plot_types.insert(*ch);
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    for (y, row) in map.iter().enumerate() {
        for (x, plot) in row.iter().enumerate() {
            for (dx, dy) in directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && nx < row.len() as isize
                    && ny >= 0
                    && ny < map.len() as isize
                    && map[ny as usize][nx as usize] == *plot
                {
                    neighbors
                        .entry((x, y))
                        .or_default()
                        .push((nx as usize, ny as usize));
                }
            }
            neighbors.entry((x, y)).or_default();
        }
    }

    for (x, y) in neighbors.keys() {
        let mut stack = vec![(*x, *y)];
        let mut region = HashSet::new();
        while let Some((x, y)) = stack.pop() {
            if in_region.contains(&(x, y)) {
                continue;
            }
            in_region.insert((x, y));
            region.insert((x, y));
            if let Some(neighbors) = neighbors.get(&(x, y)) {
                for (nx, ny) in neighbors {
                    stack.push((*nx, *ny));
                }
            }
        }
        if !region.is_empty() {
            regions.push(region);
        }
    }

    let price: usize = regions.iter().map(calculate_price).sum();
    price.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!("1930", process(input));
    }
}
