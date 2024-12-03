#[derive(Debug)]
enum Direction {
    Ascending,
    Descending,
}

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn direction(&self) -> Direction {
        if self.levels[0] < self.levels[1] {
            Direction::Ascending
        } else {
            Direction::Descending
        }
    }
    fn is_safe(&self) -> bool {
        let direction = self.direction();
        match direction {
            Direction::Ascending => self
                .levels
                .windows(2)
                .all(|window| window[0] < window[1] && window[1] <= window[0] + 3),
            Direction::Descending => self
                .levels
                .windows(2)
                .all(|window| window[0] > window[1] && window[1] >= window[0] - 3),
        }
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let reports: Vec<Report> = input.lines().map(to_report).collect();
    let mut safe_reports = 0;
    for report in &reports {
        let is_safe = report.is_safe();
        safe_reports += is_safe as i32;
    }
    safe_reports.to_string()
}

fn to_report(line: &str) -> Report {
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().expect("Should be a number"))
        .collect();

    Report { levels }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input));
    }
}
