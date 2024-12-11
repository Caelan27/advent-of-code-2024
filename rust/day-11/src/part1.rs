fn blink(mut stones: Vec<u64>, n: u64) -> Vec<u64> {
    for _ in 0..n {
        stones = stones
            .iter()
            .flat_map(|stone| {
                if *stone == 0 {
                    vec![1]
                } else if stone.to_string().len() % 2 == 0 {
                    let length = stone.to_string().len();
                    let stone_string = stone.to_string();
                    let first = stone_string[..length / 2].parse::<u64>().unwrap();
                    let second = stone_string[length / 2..].parse::<u64>().unwrap();
                    vec![first, second]
                } else {
                    vec![*stone * 2024]
                }
            })
            .collect();
    }
    stones
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let after_blinking = blink(stones, 25);
    let number_stones = after_blinking.len();

    number_stones.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "125 17";
        assert_eq!("22", process(input));
    }
}
