use std::collections::HashMap;
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

    let count_map = list2
        .iter()
        .copied()
        .fold(HashMap::new(), |mut map, number| {
            *map.entry(number).or_insert(0) += 1;
            map
        });

    let total: u32 = list1
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
