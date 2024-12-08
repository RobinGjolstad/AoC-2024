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

fn make_update_valid(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> Vec<u32> {
    // We assume the update is invalid here.
    // Iterate through the update and look for invalid numbers.
    // If an invalid number is found, move it behind the rule-numbmer being checked.

    let mut fixed_update: Vec<u32> = Vec::new();

    // First round doing fixes.
    update
        .iter()
        .enumerate()
        .for_each(|(_update_member_idx, update_member)| {
            // Store the number in the list.
            fixed_update.push(*update_member);

            // Check if any rules apply to this number.
            rules.iter().for_each(|(rule_num, rules)| {
                // Check if this rule matches the current update member.
                // If yes, check preceding numbers for validity.
                if update_member == rule_num {
                    // There is a rule for the current number.
                    // Check rules of all preceding numbers.
                    rules.iter().for_each(|rule| {
                        if fixed_update.contains(rule) {
                            // The list contains a number which is incorrectly placed.
                            // Remove it, then re-add it.
                            fixed_update.retain(|num| num != rule);
                            fixed_update.push(*rule);
                        }
                    });
                }
            });
        });

    fixed_update
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

    let mut invalid_updates = Vec::new();
    // Check each update for correctness.
    for update in updates {
        let update_invalid = update.iter().any(|num| match rules.get(num) {
            Some(ruleset) => !update_is_valid(num, ruleset, &update),
            None => false,
        });

        if update_invalid {
            invalid_updates.push(update);
        }
    }

    let mut fixed_invalid_updates = Vec::new();
    for invalid_update in invalid_updates {
        let mut update_to_fix = invalid_update.clone();
        loop {
            // Try to fix the update until it is valid.
            update_to_fix = make_update_valid(&rules, &update_to_fix);

            // Check if it is fixed.
            let update_valid = update_to_fix
                .clone()
                .iter()
                .all(|num| match rules.get(num) {
                    Some(ruleset) => update_is_valid(num, ruleset, &update_to_fix),
                    None => true,
                });

            // If the update finally is valid, store it and break out of the loop.
            if update_valid {
                fixed_invalid_updates.push(update_to_fix);
                break;
            }
        }
    }

    let center_values: Vec<u32> = fixed_invalid_updates
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
        assert_eq!(process(input), 123);
    }
}
