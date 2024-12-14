use std::collections::HashMap;
use std::collections::HashSet;

enum Direction {
    Right,
    Down,
}

fn calculate_price(region: &HashSet<(usize, usize)>) -> usize {
    let area = region.len();
    let mut corners = 0;

    let mut sorted = region.iter().collect::<Vec<_>>();
    sorted.sort_by_key(|(x, y)| (*y, *x));
    dbg!(&sorted);

    let neighbors = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let outside = sorted
        .iter()
        .filter(|(x, y)| {
            for (dx, dy) in neighbors.iter() {
                let nx = *x as isize + dx;
                let ny = *y as isize + dy;

                if nx < 0 || nx >= sorted.len() as isize || ny < 0 || ny >= sorted.len() as isize {
                    return true;
                }
                if !region.contains(&(nx as usize, ny as usize)) {
                    return true;
                }
            }
            false
        })
        .collect::<Vec<_>>();

    let direction = Direction::Right;
    for (x, y) in outside.iter() {
        let mut nx = *x as isize;
        let mut ny = *y as isize;
        let mut count = 0;
        loop {
            let (dx, dy) = match direction {
                Direction::Right => (0, 1),
                Direction::Down => (1, 0),
            };
            nx += dx;
            ny += dy;
            if nx < 0 || nx >= sorted.len() as isize || ny < 0 || ny >= sorted.len() as isize {
                break;
            }
            if !region.contains(&(nx as usize, ny as usize)) {
                break;
            }
            count += 1;
        }
        if count == 1 {
            corners += 1;
        }
    }
    dbg!(&outside);

    area * corners as usize
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

    let price: usize = regions
        .iter()
        .map(|region| {
            let character = map[region.iter().next().unwrap().1][region.iter().next().unwrap().0];
            dbg!(character);
            let price = calculate_price(region);
            dbg!(price);
            price
        })
        .sum();
    price.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        assert_eq!("436", process(input));
    }
}
