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

        stones.iter().for_each(|stone| {
            if stone.value == "0" {
                // Bump to 1
                new_stones.push(Stone {
                    value: "1".to_string(),
                })
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

                new_stones.push(Stone::new(format!("{first_num}").as_str()));
                new_stones.push(Stone::new(format!("{second_num}").as_str()));
            } else {
                // Multiply value by 2024
                let val = stone
                    .value
                    .parse::<usize>()
                    .expect("We should always be able to parse numbers");
                let stone_val = format!("{}", val * 2024);
                new_stones.push(Stone::new(&stone_val));
            }
        });

        new_stones
    }
}

fn blink(num_blink: usize, stones: &[Stone]) -> usize {
    if num_blink == 0 {
        return stones.len();
    }

    println!("Blink: {num_blink}");

    let new_stones = Stone::apply_rules(stones);
    blink(num_blink - 1, &new_stones)
}

pub fn process(input: &str) -> usize {
    let input: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let strings: Vec<&str> = input.first().unwrap().split(' ').collect();
    let stones: Vec<Stone> = strings.iter().map(|s| Stone::new(s)).collect();

    // Task says 25 blinks.
    let num_blinks = 25;

    blink(num_blinks, &stones)
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
        let res = blink(num_blink, &stones);

        assert_eq!(res, expected);
    }
}
