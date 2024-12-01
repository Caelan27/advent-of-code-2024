#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (mut list1, mut list2): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            let mut lists = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            (lists.next().unwrap(), lists.next().unwrap())
        })
        .unzip();

    list1.sort();
    list2.sort();

    let total: u32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(&a, &b)| (a as i32 - b as i32).unsigned_abs())
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
        assert_eq!("11", process(input));
    }
}
