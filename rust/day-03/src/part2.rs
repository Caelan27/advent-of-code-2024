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
        if let (Some(first), Some(second)) = (caps.get(1), caps.get(2)) {
            let first = first.as_str().parse::<u32>().ok()?;
            let second = second.as_str().parse::<u32>().ok()?;
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
