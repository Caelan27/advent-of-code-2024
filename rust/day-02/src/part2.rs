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
    fn check_safe_remove(&self, remove_index: Option<usize>) -> bool {
        let mut levels = self.levels.clone();

        if let Some(index) = remove_index {
            levels.remove(index);
        };

        let direction = update_direction(&levels);

        match direction {
            Direction::Ascending => levels
                .windows(2)
                .all(|window| window[0] < window[1] && window[1] <= window[0] + 3),

            Direction::Descending => levels
                .windows(2)
                .all(|window| window[0] > window[1] && window[1] >= window[0] - 3),
        }
    }

    fn is_safe(&self) -> bool {
        if self.check_safe_remove(None) {
            return true;
        }

        for index in 0..self.levels.len() {
            if self.check_safe_remove(Some(index)) {
                return true;
            }
        }

        false
    }
}

fn update_direction(levels: &[i32]) -> Direction {
    if levels[0] < levels[1] {
        Direction::Ascending
    } else {
        Direction::Descending
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
        assert_eq!("4", process(input));
    }
}
