#[derive(Debug, Clone)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn check_for_tree(grid: &[Vec<char>], size: u32) -> bool {
    let mut tree_pattern: Vec<(i32, i32)> = Vec::new();

    for row in 0..size {
        for col in 0..size * 2 {
            tree_pattern.push((col as i32 - (size as i32), row as i32));
        }
    }

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != '#' {
                continue;
            }
            let last = tree_pattern.iter().last().unwrap();
            for (dx, dy) in tree_pattern.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || ny < 0 || nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                    break;
                }

                if grid[ny as usize][nx as usize] != '#' {
                    break;
                }

                if (*dx, *dy) == *last {
                    return true;
                }
            }
        }
    }
    false
}

fn print_robots(robots: &Vec<Robot>, grid_size: (i32, i32)) {
    let mut grid = vec![vec!['.'; grid_size.0 as usize]; grid_size.1 as usize];
    for robot in robots {
        grid[robot.position.1 as usize][robot.position.0 as usize] = '#';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }

    println!();
    println!();
    println!();
}

fn parse(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut position = parts.next().unwrap().split('=').nth(1).unwrap().split(',');
            let x = position.next().unwrap().parse().unwrap();
            let y = position.next().unwrap().parse().unwrap();
            let mut velocity = parts.next().unwrap().split('=').nth(1).unwrap().split(',');
            let vx = velocity.next().unwrap().parse().unwrap();
            let vy = velocity.next().unwrap().parse().unwrap();
            Robot {
                position: (x, y),
                velocity: (vx, vy),
            }
        })
        .collect()
}

fn find_tree(robots: &mut [Robot], grid_size: (i32, i32)) -> Option<i32> {
    let mut second = 0;
    for _ in 0..10000 {
        second += 1;
        for robot in robots.iter_mut() {
            robot.position.0 += robot.velocity.0;
            robot.position.0 = robot.position.0.rem_euclid(grid_size.0);

            robot.position.1 += robot.velocity.1;
            robot.position.1 = robot.position.1.rem_euclid(grid_size.1);
        }

        let mut grid = vec![vec!['.'; grid_size.0 as usize]; grid_size.1 as usize];

        let robots = robots.to_vec();
        for robot in robots.iter() {
            grid[robot.position.1 as usize][robot.position.0 as usize] = '#';
        }

        if check_for_tree(&grid, 5) {
            print_robots(&robots, grid_size);
            return Some(second);
        }
    }
    None
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut robots = parse(input);
    let grid_size = (101, 103);

    let result = find_tree(&mut robots, grid_size).unwrap();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", process(input));
    }
}
