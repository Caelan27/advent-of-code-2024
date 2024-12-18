#[derive(Debug)]
struct Equation {
    value: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn check_equation(equation: &Equation) -> bool {
    let num_operators = equation.numbers.len() as u32 - 1;
    // There are 3^n combinations of operators, since each operator can be one ternary digit
    let total_combinations = 3_u32.pow(num_operators);

    for combination in 0..total_combinations {
        let mut operators = Vec::new();
        for shift in 0..num_operators {
            // Dividing by 3^shift is basically a ternary bit shift
            // Taking the modulo 3 gives us the value of the last ternary digit
            let operator = match (combination / 3_u32.pow(shift)) % 3 {
                0 => Operator::Add,
                1 => Operator::Multiply,
                2 => Operator::Concatenate,
                _ => unreachable!(),
            };
            operators.push(operator);
        }
        if calculate(equation, &operators) == equation.value {
            return true;
        }
    }
    false
}

fn calculate(equation: &Equation, operators: &[Operator]) -> u64 {
    let mut result = equation.numbers[0];
    for (index, operator) in operators.iter().enumerate() {
        match operator {
            Operator::Add => result += equation.numbers[index + 1],
            Operator::Multiply => result *= equation.numbers[index + 1],
            Operator::Concatenate => {
                result = format!("{}{}", result, equation.numbers[index + 1])
                    .parse()
                    .unwrap()
            }
        }
    }
    result
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let equations = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let value: u64 = parts.next().unwrap().parse().expect("Invalid value");
            let numbers = parts
                .next()
                .unwrap()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            Equation { value, numbers }
        })
        .collect::<Vec<_>>();

    let mut total = 0;
    for equation in equations.iter() {
        if check_equation(equation) {
            total += equation.value;
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input));
    }
}
