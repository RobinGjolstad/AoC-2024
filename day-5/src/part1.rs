use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_rules(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(complete::u32, tag("|"), complete::u32)(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), complete::u32)(input)
}

#[allow(unreachable_code)]
fn update_is_valid(num: &u32, rules: &[u32], update: &[u32]) -> bool {
    debug_assert!(
        update.contains(num),
        "An update must contain the number one wants to verify rules against."
    );

    // Find self in `update`.
    match update.iter().position(|x| x == num) {
        Some(pos) => {
            // Check all numbers before this index against the rules.
            for update_member in &update[0..pos] {
                if rules.contains(update_member) {
                    // The rules contains this number.
                    return false;
                }
            }
        }
        None => {
            // The number we're checking our rules against isn't in the update list.
            // Something is very wrong.
            panic!("Checking rules on an update which doesn't contain the target number.");
        }
    }

    true
}

pub fn process(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();

    // Loop trough the entire input.
    // Start by storing rules.
    // Upon encountering an empty line, swap to storing updates.
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut parsing_rules = true;
    for line in lines {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        // Grab rules.
        if parsing_rules {
            let (_, (rule_number, trail)) = parse_rules(line).unwrap();
            rules.entry(rule_number).or_default().push(trail);
        } else {
            // We're done with rules, so now we grab updates
            let (_, update) = parse_updates(line).unwrap();
            updates.push(update);
        }
    }

    let mut valid_updates = Vec::new();
    // Check each update for correctness.
    for update in updates {
        let update_invalid = update.iter().any(|num| match rules.get(num) {
            Some(ruleset) => !update_is_valid(num, ruleset, &update),
            None => false,
        });

        if !update_invalid {
            valid_updates.push(update);
        }
    }

    let center_values: Vec<u32> = valid_updates
        .into_iter()
        .map(|update| update[update.len() / 2])
        .collect();

    let center_value_sum: u32 = center_values.into_iter().sum();

    center_value_sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"47|53
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
97,13,75,29,47"#;
        assert_eq!(process(input), 143);
    }
}
