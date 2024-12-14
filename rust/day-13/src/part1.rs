#[derive(Debug)]
struct Machine {
    a_button_x: i32,
    a_button_y: i32,
    b_button_x: i32,
    b_button_y: i32,
    prize_x: i32,
    prize_y: i32,
}

#[derive(Debug)]
enum Solution {
    Solvable { num_tokens: i32 },
    Unsolvable,
}

fn win(game: Machine) -> Solution {
    let a_button_x = game.a_button_x as f64;
    let a_button_y = game.a_button_y as f64;
    let b_button_x = game.b_button_x as f64;
    let b_button_y = game.b_button_y as f64;
    let prize_x = game.prize_x as f64;
    let prize_y = game.prize_y as f64;

    let determinant = a_button_x * b_button_y - a_button_y * b_button_x;
    if determinant == 0.0 {
        return Solution::Unsolvable;
    }

    let adjoint: [[f64; 2]; 2] = [[b_button_y, -b_button_x], [-a_button_y, a_button_x]];

    let a = (adjoint[0][0] * prize_x + adjoint[0][1] * prize_y) / determinant;
    let b = (adjoint[1][0] * prize_x + adjoint[1][1] * prize_y) / determinant;
    if a < 0.0 || b < 0.0 || a.fract() != 0.0 || b.fract() != 0.0 {
        Solution::Unsolvable
    } else {
        Solution::Solvable {
            num_tokens: (a * 3.0 + b) as i32,
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine| {
            let lines = machine
                .lines()
                .map(|line| {
                    line.chars()
                        .filter(|c| c.is_ascii_digit() || *c == ',')
                        .collect::<String>()
                        .split(',')
                        .map(|part| part.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();

            let a_button_x = lines[0][0];
            let a_button_y = lines[0][1];
            let b_button_x = lines[1][0];
            let b_button_y = lines[1][1];
            let prize_x = lines[2][0];
            let prize_y = lines[2][1];

            Machine {
                a_button_x,
                a_button_y,
                b_button_x,
                b_button_y,
                prize_x,
                prize_y,
            }
        })
        .collect::<Vec<Machine>>()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let machines = parse(input);
    let mut total = 0;
    for machine in machines {
        let solution = win(machine);
        dbg!(&solution);
        match solution {
            Solution::Solvable { num_tokens } => {
                total += num_tokens;
                dbg!(total, num_tokens);
            }
            Solution::Unsolvable => {}
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input));
    }
}
