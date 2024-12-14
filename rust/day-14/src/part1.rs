#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
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

fn move_robots(robots: &mut [Robot], grid_size: (i32, i32), seconds: i32) {
    for robot in robots.iter_mut() {
        robot.position.0 += robot.velocity.0 * seconds;
        robot.position.0 = robot.position.0.rem_euclid(grid_size.0);

        robot.position.1 += robot.velocity.1 * seconds;
        robot.position.1 = robot.position.1.rem_euclid(grid_size.1);
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let mut robots = parse(input);
    let grid_size = (101, 103);
    let seconds = 100;

    move_robots(&mut robots, grid_size, seconds);
    let mut robots_in_quadrants: [Vec<Robot>; 4] = [vec![], vec![], vec![], vec![]];

    let mut safety_factor = 1;

    for robot in robots {
        if robot.position.0 < grid_size.0 / 2 && robot.position.1 < grid_size.1 / 2 {
            robots_in_quadrants[0].push(robot);
        } else if robot.position.0 > grid_size.0 / 2 && robot.position.1 < grid_size.1 / 2 {
            robots_in_quadrants[1].push(robot);
        } else if robot.position.0 < grid_size.0 / 2 && robot.position.1 > grid_size.1 / 2 {
            robots_in_quadrants[2].push(robot);
        } else if robot.position.0 > grid_size.0 / 2 && robot.position.1 > grid_size.1 / 2 {
            robots_in_quadrants[3].push(robot);
        }
    }

    for quadrant in robots_in_quadrants.iter() {
        safety_factor *= quadrant.len() as i32;
    }

    safety_factor.to_string()
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
