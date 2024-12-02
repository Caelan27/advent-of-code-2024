use std::collections::HashMap;
#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (left_value, right_value): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut lists = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().expect("Should be a number"));
            (
                lists.next().expect("Should have a left value"),
                lists.next().expect("Should have a right value"),
            )
        })
        .unzip();

    let count_map = right_value
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, number| {
            *map.entry(number).or_insert(0) += 1;
            map
        });

    let total: u32 = left_value
        .iter()
        .map(|&left| left * count_map.get(&left).unwrap_or(&0))
        .sum();

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input));
    }
}
