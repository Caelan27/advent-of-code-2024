use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Before(u32),
    After(u32),
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (rules, updates) = parse_input(input);
    let mut rules_map: HashMap<u32, Vec<Rule>> = std::collections::HashMap::new();
    for (before, after) in &rules {
        rules_map
            .entry(*before)
            .and_modify(|page| page.push(Rule::After(*after)))
            .or_insert(vec![Rule::After(*after)]);
        rules_map
            .entry(*after)
            .and_modify(|page| page.push(Rule::Before(*before)))
            .or_insert(vec![Rule::Before(*before)]);
    }

    let mut total = 0;
    for update in updates.iter() {
        let mut before_current_in_update: Vec<u32> = vec![];
        let mut after_in_update = update.clone();
        let mut correct = true;
        for page in update {
            if correct {
                if let Some(rules) = rules_map.get(page) {
                    for rule in rules {
                        if let Rule::Before(before) = rule {
                            if after_in_update.contains(before) {
                                correct = false;
                                sort(update.clone(), &rules_map);
                            }
                        }
                        if let Rule::After(after) = rule {
                            if before_current_in_update.contains(after) {
                                correct = false;
                                sort(update.clone(), &rules_map);
                            }
                        }
                    }
                }
                after_in_update.remove(0);
                before_current_in_update.push(*page);
            } else {
                break;
            }
        }
        if !correct {
            let sorted = sort(update.clone(), &rules_map);
            total += sorted[(sorted.len() - 1) / 2]
        }
    }

    total.to_string()
}

fn sort(update: Vec<u32>, rules_map: &HashMap<u32, Vec<Rule>>) -> Vec<u32> {
    let dependency_graph = rules_map
        .iter()
        .fold(HashMap::new(), |mut acc, (&page, rules)| {
            if update.contains(&page) {
                let filtered_rules: Vec<Rule> = rules
                    .iter()
                    .filter(|rule| match rule {
                        Rule::Before(before) => update.contains(before),
                        Rule::After(after) => update.contains(after),
                    })
                    .cloned()
                    .collect();
                acc.insert(page, filtered_rules);
            }
            acc
        });

    let pages_before_map =
        dependency_graph
            .iter()
            .fold(HashMap::new(), |mut acc, (&page, rules)| {
                for rule in rules {
                    if let Rule::Before(_before) = rule {
                        acc.entry(page).and_modify(|pages| *pages += 1).or_insert(1);
                    } else {
                        acc.entry(page).or_insert(0);
                    }
                }
                acc
            });

    let mut sorted: Vec<u32> = vec![0; update.len()];
    for (page, number_before) in pages_before_map.iter() {
        sorted[*number_before as usize] = *page;
    }

    sorted
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut input_parts = input.split("\n\n");
    let rules: Vec<(u32, u32)> = input_parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<(u32, u32)>>();
    let updates: Vec<Vec<u32>> = input_parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|part| part.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input));
    }
}
