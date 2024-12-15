#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum SpaceType {
    Wall,
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
}

fn parse(input: &str) -> (Vec<Vec<SpaceType>>, Vec<Direction>) {
    let mut map = vec![];
    let mut directions = vec![];

    let mut sections = input.split("\n\n");
    let map_section = sections.next().unwrap();
    let directions_section = sections.next().unwrap();
    for line in map_section.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let (first, second) = match c {
                '#' => (SpaceType::Wall, SpaceType::Wall),
                '.' => (SpaceType::Empty, SpaceType::Empty),
                '@' => (SpaceType::Robot, SpaceType::Empty),
                'O' => (SpaceType::BoxLeft, SpaceType::BoxRight),
                _ => panic!("unexpected character in map: {}", c),
            };
            row.push(first);
            row.push(second);
        }
        map.push(row);
    }

    directions_section.chars().for_each(|c| {
        let direction = match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        };
        if let Some(direction) = direction {
            directions.push(direction);
        }
    });

    (map, directions)
}

fn move_robot(
    map: &mut Vec<Vec<SpaceType>>,
    robot_position: &mut (usize, usize),
    direction: Direction,
) {
    let (robot_x, robot_y) = robot_position;

    let (dx, dy): (isize, isize) = match direction {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };

    let nx = (*robot_x as isize + dx) as usize;
    let ny = (*robot_y as isize + dy) as usize;

    match &map[ny][nx] {
        SpaceType::Wall => (),
        SpaceType::Empty => {
            map[*robot_y][*robot_x] = SpaceType::Empty;
            map[ny][nx] = SpaceType::Robot;
            *robot_x = nx;
            *robot_y = ny;
        }
        SpaceType::BoxLeft => {
            push_box(map, (nx, ny), &direction);
            if let SpaceType::Empty = map[ny][nx] {
                map[*robot_y][*robot_x] = SpaceType::Empty;
                map[ny][nx] = SpaceType::Robot;
                *robot_x = nx;
                *robot_y = ny;
            }
        }
        SpaceType::BoxRight => {
            push_box(map, (nx, ny), &direction);
            if let SpaceType::Empty = map[ny][nx] {
                map[*robot_y][*robot_x] = SpaceType::Empty;
                map[ny][nx] = SpaceType::Robot;
                *robot_x = nx;
                *robot_y = ny;
            }
        }

        SpaceType::Robot => {}
    }
}

fn push_box(map: &mut Vec<Vec<SpaceType>>, box_position: (usize, usize), direction: &Direction) {
    let (box_x, box_y) = box_position;
    let left_x = match map[box_y][box_x] {
        SpaceType::BoxLeft => box_x,
        SpaceType::BoxRight => box_x - 1,
        _ => panic!("expected box"),
    };
    let right_x = left_x + 1;

    match direction {
        Direction::Left => {
            if let SpaceType::Empty = map[box_y][left_x - 1] {
                map[box_y][left_x - 1] = SpaceType::BoxLeft;
                map[box_y][left_x] = SpaceType::BoxRight;
                map[box_y][right_x] = SpaceType::Empty;
            } else if let SpaceType::BoxRight = map[box_y][left_x - 1] {
                push_box(map, (left_x - 1, box_y), direction);
                if let SpaceType::Empty = map[box_y][left_x - 1] {
                    map[box_y][left_x - 1] = SpaceType::BoxLeft;
                    map[box_y][left_x] = SpaceType::BoxRight;
                    map[box_y][right_x] = SpaceType::Empty;
                }
            }
        }
        Direction::Right => {
            if let SpaceType::Empty = map[box_y][right_x + 1] {
                map[box_y][right_x + 1] = SpaceType::BoxRight;
                map[box_y][right_x] = SpaceType::BoxLeft;
                map[box_y][left_x] = SpaceType::Empty;
            } else if let SpaceType::BoxLeft = map[box_y][right_x + 1] {
                push_box(map, (right_x + 1, box_y), direction);
                if let SpaceType::Empty = map[box_y][right_x + 1] {
                    map[box_y][right_x + 1] = SpaceType::BoxRight;
                    map[box_y][right_x] = SpaceType::BoxLeft;
                    map[box_y][left_x] = SpaceType::Empty;
                }
            }
        }

        Direction::Up => {
            let ny = box_y - 1;
            match map[ny][left_x] {
                SpaceType::Empty => {
                    if let SpaceType::Empty = map[ny][right_x] {
                        map[ny][left_x] = SpaceType::BoxLeft;
                        map[ny][right_x] = SpaceType::BoxRight;
                        map[box_y][left_x] = SpaceType::Empty;
                        map[box_y][right_x] = SpaceType::Empty;
                    } else if map[ny][right_x] == SpaceType::BoxRight
                        || map[ny][right_x] == SpaceType::BoxLeft
                    {
                        push_box(map, (right_x, ny), direction);
                        if let SpaceType::Empty = map[ny][right_x] {
                            map[ny][left_x] = SpaceType::BoxLeft;
                            map[ny][right_x] = SpaceType::BoxRight;
                            map[box_y][left_x] = SpaceType::Empty;
                            map[box_y][right_x] = SpaceType::Empty;
                        }
                    }
                }
                SpaceType::Robot => (),
                SpaceType::Wall => (),
                _ => match map[ny][right_x] {
                    SpaceType::Empty => {
                        push_box(map, (left_x, ny), direction);
                        if let SpaceType::Empty = map[ny][left_x] {
                            map[ny][left_x] = SpaceType::BoxLeft;
                            map[ny][right_x] = SpaceType::BoxRight;
                            map[box_y][left_x] = SpaceType::Empty;
                            map[box_y][right_x] = SpaceType::Empty;
                        }
                    }
                    SpaceType::Robot => (),
                    SpaceType::Wall => (),
                    _ => {
                        let map_copy = map.clone();
                        push_box(map, (left_x, ny), direction);
                        if map[ny][right_x] != SpaceType::Empty {
                            push_box(map, (right_x, ny), direction);
                        }
                        if map[ny][left_x] == SpaceType::Empty
                            && map[ny][right_x] == SpaceType::Empty
                        {
                            map[ny][left_x] = SpaceType::BoxLeft;
                            map[ny][right_x] = SpaceType::BoxRight;
                            map[box_y][left_x] = SpaceType::Empty;
                            map[box_y][right_x] = SpaceType::Empty;
                        } else {
                            *map = map_copy;
                        }
                    }
                },
            }
        }
        Direction::Down => {
            let ny = box_y + 1;
            match map[ny][left_x] {
                SpaceType::Empty => {
                    if let SpaceType::Empty = map[ny][right_x] {
                        map[ny][left_x] = SpaceType::BoxLeft;
                        map[ny][right_x] = SpaceType::BoxRight;
                        map[box_y][left_x] = SpaceType::Empty;
                        map[box_y][right_x] = SpaceType::Empty;
                    } else if map[ny][right_x] == SpaceType::BoxRight
                        || map[ny][right_x] == SpaceType::BoxLeft
                    {
                        push_box(map, (right_x, ny), direction);
                        if let SpaceType::Empty = map[ny][right_x] {
                            map[ny][left_x] = SpaceType::BoxLeft;
                            map[ny][right_x] = SpaceType::BoxRight;
                            map[box_y][left_x] = SpaceType::Empty;
                            map[box_y][right_x] = SpaceType::Empty;
                        }
                    }
                }
                SpaceType::Robot => (),
                SpaceType::Wall => (),
                _ => match map[ny][right_x] {
                    SpaceType::Empty => {
                        push_box(map, (left_x, ny), direction);
                        if let SpaceType::Empty = map[ny][left_x] {
                            map[ny][left_x] = SpaceType::BoxLeft;
                            map[ny][right_x] = SpaceType::BoxRight;
                            map[box_y][left_x] = SpaceType::Empty;
                            map[box_y][right_x] = SpaceType::Empty;
                        }
                    }
                    SpaceType::Robot => (),
                    SpaceType::Wall => (),
                    _ => {
                        let map_copy = map.clone();
                        push_box(map, (left_x, ny), direction);
                        if map[ny][right_x] != SpaceType::Empty {
                            push_box(map, (right_x, ny), direction);
                        }
                        if map[ny][left_x] == SpaceType::Empty
                            && map[ny][right_x] == SpaceType::Empty
                        {
                            map[ny][left_x] = SpaceType::BoxLeft;
                            map[ny][right_x] = SpaceType::BoxRight;
                            map[box_y][left_x] = SpaceType::Empty;
                            map[box_y][right_x] = SpaceType::Empty;
                        } else {
                            *map = map_copy;
                        }
                    }
                },
            }
        }
    }
    /*
    match &map[ny][nx] {
        SpaceType::Wall => (),
        SpaceType::Empty => {
            map[box_y][box_x] = SpaceType::Empty;
            map[ny][nx] = SpaceType::Box;
        }
        SpaceType::Box => {
            push_box(map, (nx, ny), direction);
            if let SpaceType::Empty = map[ny][nx] {
                map[box_y][box_x] = SpaceType::Empty;
                map[ny][nx] = SpaceType::Box;
            }
        }

        SpaceType::Robot => {}
    }
    */
}

fn print_map(map: &[Vec<SpaceType>]) {
    for row in map {
        for space in row {
            let c = match space {
                SpaceType::Wall => '#',
                SpaceType::Empty => '.',
                SpaceType::Robot => '@',
                SpaceType::BoxLeft => '[',
                SpaceType::BoxRight => ']',
            };
            print!("{}", c);
        }
        println!();
    }
}

fn gps(map: &[Vec<SpaceType>]) -> usize {
    let mut gps_sum = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if let SpaceType::BoxLeft = space {
                gps_sum += 100 * y + x;
            }
        }
    }
    gps_sum
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (mut map, directions) = parse(input);
    let mut robot_position = (0, 0);

    for (y, row) in map.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if let SpaceType::Robot = space {
                robot_position = (x, y);
            }
        }
    }

    for direction in directions {
        move_robot(&mut map, &mut robot_position, direction);
    }

    let gps_sum = gps(&map);

    gps_sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", process(input));
    }
}
