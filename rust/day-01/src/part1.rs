#[tracing::instrument]
pub fn process(input: &str) -> String {
    // Parse the input into two lists of values
    let (mut left_values, mut right_values): (Vec<u32>, Vec<u32>) = input
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

    // Sort the lists
    left_values.sort_unstable();
    right_values.sort_unstable();

    // Calculate the total difference between the two lists
    let total: u32 = left_values
        .iter()
        .zip(right_values.iter())
        .map(|(&a, &b)| (a as i32 - b as i32).unsigned_abs())
        .sum();

    // Convert the result to a string
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
        assert_eq!("11", process(input));
    }
}
