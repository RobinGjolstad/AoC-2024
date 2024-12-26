use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Stone {
    value: String,
}

impl Stone {
    fn new(input: &str) -> Self {
        Self {
            value: input.to_string(),
        }
    }

    fn apply_rules(stones: &[Stone]) -> Vec<Stone> {
        let mut new_stones: Vec<Stone> = Vec::new();

        for stone in stones.iter() {
            // if let Some(known_stones) = known_combinations.get(&stone.value) {
            //     new_stones.extend(known_stones.clone());
            //     continue;
            // }

            if stone.value == "0" {
                // Bump to 1
                let stones_to_push = vec![Stone {
                    value: "1".to_string(),
                }];
                // known_combinations.insert(stone.value.clone(), stones_to_push.clone());
                new_stones.extend(stones_to_push);
            } else if stone.value.len() % 2 == 0 {
                // Split string in middle.
                let middle = stone.value.len() / 2;

                let first_string = &stone.value[0..middle];
                let first_num = first_string
                    .parse::<usize>()
                    .expect("All numbers are parsable");
                let second_string = &stone.value[middle..stone.value.len()];
                let second_num = second_string
                    .parse::<usize>()
                    .expect("All numbers are parsable");

                let stones_to_push = vec![
                    Stone::new(format!("{first_num}").as_str()),
                    Stone::new(format!("{second_num}").as_str()),
                ];
                // known_combinations.insert(stone.value.clone(), stones_to_push.clone());
                new_stones.extend(stones_to_push);
            } else {
                // Multiply value by 2024
                let val = stone
                    .value
                    .parse::<usize>()
                    .expect("We should always be able to parse numbers");
                let stone_val = format!("{}", val * 2024);
                let stones_to_push = vec![Stone::new(&stone_val)];
                // known_combinations.insert(stone.value.clone(), stones_to_push.clone());
                new_stones.extend(stones_to_push);
            }
        }

        new_stones
    }
}

fn blink(
    num_blink: usize,
    stones: &[Stone],
    known_combinations: &mut HashMap<(usize, Stone), usize>,
) -> usize {
    // Recursive base case.
    if num_blink == 0 {
        return stones.len();
    }

    // Recursive stone handling.
    let sum = stones
        .iter()
        .map(|stone| {
            if let Some(num) = known_combinations.get(&(num_blink, stone.clone())) {
                println!("Found known solution: {} => {}", stone.value, num);
                *num
            } else {
                // Since we apply rules, then blink on the new stones, we perform a depth-first process.
                // This should ideally avoid endless RAM usage since we will hopefully avoid keeping all
                // stones in memory at once.
                let new_stones = Stone::apply_rules(&[stone.clone()]);
                let num = blink(num_blink - 1, &new_stones, known_combinations);
                known_combinations.insert((num_blink, stone.clone()), num);
                num
            }
        })
        .sum();

    sum
}

pub fn process(input: &str) -> usize {
    let input: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let strings: Vec<&str> = input.first().unwrap().split(' ').collect();
    let stones: Vec<Stone> = strings.iter().map(|s| Stone::new(s)).collect();
    let mut stone_rule_lookuptable = HashMap::new();

    // Task says 75 blinks.
    let num_blinks = 75;

    blink(num_blinks, &stones, &mut stone_rule_lookuptable)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // #[test]
    // fn test_process() {
    //     let input = "";
    //     assert_eq!(process(input), 0);
    // }

    #[rstest]
    #[case(
        &["125", "17"],
        &["253000", "1", "7"]
    )]
    #[case(
        &["253000", "1", "7"],
        &["253", "0", "2024", "14168"]
    )]
    #[case(
        &["253", "0", "2024", "14168"],
        &["512072", "1", "20", "24", "28676032"]
    )]
    #[case(
        &["512072", "1", "20", "24", "28676032"],
        &["512", "72", "2024", "2", "0", "2", "4", "2867", "6032"]
    )]
    #[case(
        &["512", "72", "2024", "2", "0", "2", "4", "2867", "6032"],
        &["1036288", "7", "2", "20", "24", "4048", "1", "4048", "8096", "28", "67", "60", "32"]
    )]
    #[case(
        &["1036288", "7", "2", "20", "24", "4048", "1", "4048", "8096", "28", "67", "60", "32"],
        &["2097446912", "14168", "4048", "2", "0", "2", "4", "40", "48", "2024", "40", "48", "80", "96", "2", "8", "6", "7", "6", "0", "3", "2"]
    )]
    fn test_stone_rules(#[case] input_stones: &[&str], #[case] expected_stones: &[&str]) {
        let input_stones: Vec<Stone> = input_stones.iter().map(|val| Stone::new(val)).collect();

        let expected_stones: Vec<Stone> =
            expected_stones.iter().map(|val| Stone::new(val)).collect();

        assert_eq!(Stone::apply_rules(&input_stones), expected_stones);
    }

    #[rstest]
    #[case(
        &["125", "17"], // Stones
        1,              // Num blink
        3               // Expected
    )]
    #[case(
        &["125", "17"], // Stones
        2,              // Num blink
        4               // Expected
    )]
    #[case(
        &["125", "17"], // Stones
        3,              // Num blink
        5               // Expected
    )]
    #[case(
        &["125", "17"], // Stones
        4,              // Num blink
        9               // Expected
    )]
    #[case(
        &["125", "17"], // Stones
        5,              // Num blink
        13              // Expected
    )]
    #[case(
        &["125", "17"], // Stones
        6,              // Num blink
        22              // Expected
    )]
    fn test_blinks(
        #[case] input_stones: &[&str],
        #[case] num_blink: usize,
        #[case] expected: usize,
    ) {
        let stones: Vec<Stone> = input_stones.iter().map(|s| Stone::new(s)).collect();
        let mut stone_lut = HashMap::new();
        let res = blink(num_blink, &stones, &mut stone_lut);

        assert_eq!(res, expected);
    }
}

