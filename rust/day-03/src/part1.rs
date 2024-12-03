use regex::Regex;
#[tracing::instrument]
pub fn process(input: &str) -> String {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = re
        .captures_iter(input)
        .map(|caps| {
            let first = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let second = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            first * second
        })
        .sum::<u32>();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input));
    }
}
