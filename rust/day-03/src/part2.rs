use regex::Regex;

#[derive(Debug)]
enum MatchResult {
    Mul(u32, u32),
    Do,
    Dont,
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let regexed = re.captures_iter(input).filter_map(|caps| {
        if caps.get(1).is_some() {
            let first = caps.get(1).unwrap().as_str().parse().unwrap();
            let second = caps.get(2).unwrap().as_str().parse().unwrap();
            Some(MatchResult::Mul(first, second))
        } else if caps.get(0)?.as_str() == "do()" {
            Some(MatchResult::Do)
        } else if caps.get(0)?.as_str() == "don't()" {
            Some(MatchResult::Dont)
        } else {
            None
        }
    });

    let mut doing = true;
    let mut result = 0;
    for function in regexed {
        match function {
            MatchResult::Mul(first, second) if doing => result += first * second,
            MatchResult::Do => doing = true,
            MatchResult::Dont => doing = false,
            _ => {}
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input));
    }
}
