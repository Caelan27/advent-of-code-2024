use std::collections::HashMap;

fn blink(stones: Vec<u64>, n: u64) -> Vec<u64> {
    let mut stones_map = stones.iter().fold(HashMap::new(), |mut acc, &stone| {
        *acc.entry(stone).or_insert(0) += 1;
        acc
    });

    for _ in 0..n {
        stones_map = stones_map
            .iter()
            .fold(HashMap::new(), |mut acc, (stone, count)| {
                if *stone == 0 {
                    *acc.entry(1).or_insert(0) += *count;
                } else if stone.to_string().len() % 2 == 0 {
                    let length = stone.to_string().len();
                    let stone_string = stone.to_string();
                    let first = stone_string[..length / 2].parse::<u64>().unwrap();
                    let second = stone_string[length / 2..].parse::<u64>().unwrap();
                    *acc.entry(first).or_insert(0) += *count;
                    *acc.entry(second).or_insert(0) += *count;
                } else {
                    *acc.entry(*stone * 2024).or_insert(0) += *count;
                }
                acc
            });
    }
    stones_map.values().copied().collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let after_blinking = blink(stones, 75);
    let number_stones = after_blinking.iter().sum::<u64>();

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
