use std::collections::HashSet;

use rayon::{
    iter::IntoParallelIterator,
    prelude::{IntoParallelRefIterator, ParallelIterator},
};

const SAFE_LIMIT: usize = 3;

#[derive(Debug)]
enum Direction {
    Ascending,
    Descending,
}

pub fn process(input: &str) -> usize {
    let numbers = get_numbers_from_input(input);

    let sum = numbers
        .iter()
        .filter_map(|nums| is_report_safe(nums, 2).then_some(1))
        .count();

    sum
}

fn get_numbers_from_input(input: &str) -> Vec<Vec<usize>> {
    // Split the input into lines.
    let lines = input.lines();

    // Strip whitespace at each end.
    let lines: Vec<&str> = lines.into_iter().map(|line| line.trim()).collect();

    // Remove empty lines.
    let lines: Vec<&str> = lines
        .par_iter()
        .filter_map(|line| if !line.is_empty() { Some(*line) } else { None })
        .collect();

    // Convert each line into a vector of numbers.
    let numbers: Vec<Vec<usize>> = lines
        .par_iter()
        .map(|line| {
            // Split each line into subsections.
            let parts: Vec<&str> = line.split_whitespace().collect();
            let numbers: Vec<usize> = parts
                .par_iter()
                .map(|num| num.parse::<usize>().unwrap())
                .collect();
            numbers
        })
        .collect();

    numbers
}

fn number_of_duplicates_in_list(list: &[usize]) -> usize {
    let mut unique_number_set = HashSet::new();
    list.iter()
        .filter_map(|num| (!unique_number_set.insert(num)).then_some(1))
        .count()
}

fn is_report_safe(numbers: &[usize], attempts_remaining: usize) -> bool {
    if attempts_remaining == 0 {
        return false;
    }

    // We should now allow _one_ fail, which should be removed, but the rules must still apply
    // overall.
    let mut direction: Option<Direction> = None;

    let mut first_number_index = 0;
    let mut second_number_index = 1;

    let mut first_num = numbers.get(first_number_index);
    let mut second_num = numbers.get(second_number_index);

    while first_num.is_some() && second_num.is_some() {
        let first = first_num.unwrap();
        let second = second_num.unwrap();

        // Indicates if anything in this loop iteration has been flagged as unsafe.
        // This shall trigger a retry with each of the current numbers removed.
        let mut iteration_safe = true;

        // First check for duplicates.
        // if number_of_duplicates_in_list(numbers) > 0 {
        //     println!("List contained duplicates.");
        //     iteration_safe = false;
        // }

        // Determine the direction of the numbers.

        if iteration_safe && direction.is_none() {
            direction = match first.cmp(second) {
                std::cmp::Ordering::Less => Some(Direction::Ascending),
                std::cmp::Ordering::Greater => Some(Direction::Descending),
                std::cmp::Ordering::Equal => {
                    println!("Determining direction failed due to duplicates.");
                    iteration_safe = false;
                    None
                }
            };
        }

        // Check if the difference between the two numbers is greater than the safe limit.
        if iteration_safe {
            match direction {
                Some(Direction::Ascending) => {
                    if second <= first {
                        // We were ascending, but the second number was smaller or equal to the first.
                        // Unsafe.
                        println!("Ascending, but second number was smaller or equal to the first.");
                        iteration_safe = false;
                    } else if usize::abs_diff(*first, *second) > SAFE_LIMIT {
                        // The difference between the two numbers is greater than the safe limit.
                        println!(
                            "Difference between the two numbers was greater than the safe limit."
                        );
                        iteration_safe = false;
                    }
                }
                Some(Direction::Descending) => {
                    if second >= first {
                        // We were descending, but the second number was greater or equal to the first.
                        // Unsafe.
                        println!(
                            "Descending, but second number was greater or equal to the first."
                        );
                        iteration_safe = false;
                    } else if usize::abs_diff(*first, *second) > SAFE_LIMIT {
                        // The difference between the two numbers is greater than the safe limit.
                        println!(
                            "Difference between the two numbers was greater than the safe limit."
                        );
                        iteration_safe = false;
                    }
                }
                _ => panic!("Direction was not determined."),
            }
        }

        if !iteration_safe && (attempts_remaining - 1 > 0) {
            // Try once again with each number in the list removed to check if we can bounce back
            // from a single issue.

            println!("Retrying with original list: {:?}", numbers);

            let any_success = (0..numbers.len())
                .collect::<Vec<usize>>()
                .iter()
                .any(|index| {
                    let mut vec_with_num_removed: Vec<usize> = numbers.to_vec();
                    vec_with_num_removed.remove(*index);

                    // Only allow a single attempt. No infinite recursion here!
                    is_report_safe(&vec_with_num_removed, 1)
                });

            if any_success {
                return true;
            }
        }

        first_number_index += 1;
        second_number_index += 1;

        // Re-aqcuire the numbers.
        // Easier to handle the loop like this.
        first_num = numbers.get(first_number_index);
        second_num = numbers.get(second_number_index);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#;
        assert_eq!(process(input), 4);
    }

    #[test]
    fn test_does_list_contain_duplicates() {
        // A list of numbers with no duplicates.
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let list_has_duplicates = number_of_duplicates_in_list(&list);
        assert_eq!(list_has_duplicates, 0);

        // A list with all duplicates.
        let list = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let list_has_duplicates = number_of_duplicates_in_list(&list);
        assert_eq!(list_has_duplicates, list.len() - 1);

        // A list with duplicate at the end.
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1];
        let list_has_duplicates = number_of_duplicates_in_list(&list);
        assert_eq!(list_has_duplicates, 1);
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_7_6_4_2_1() {
        let numbers = vec![vec![7, 6, 4, 2, 1]];
        assert!(is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_1_2_7_8_9() {
        let numbers = vec![vec![1, 2, 7, 8, 9]];
        assert!(!is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_9_7_6_2_1() {
        let numbers = vec![vec![9, 7, 6, 2, 1]];
        assert!(!is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_1_3_2_4_5() {
        let numbers = vec![vec![1, 3, 2, 4, 5]];
        assert!(is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_8_6_4_4_1() {
        let numbers = vec![vec![8, 6, 4, 4, 1]];
        assert!(is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_1_3_6_7_9() {
        let numbers = vec![vec![1, 3, 6, 7, 9]];
        assert!(is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_20_21_24_25_27_29_27() {
        let numbers = vec![vec![20, 21, 24, 25, 27, 29, 27]];
        assert!(is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_6_5_7_8_11_11() {
        let numbers = vec![vec![6, 5, 7, 8, 11, 11]];
        assert!(!is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_17_10_9_6_6_2() {
        let numbers = vec![vec![17, 10, 9, 6, 6, 2]];
        assert!(!is_report_safe(&numbers[0], 2));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_10_8_12_14_15_17() {
        let numbers = vec![vec![10, 8, 12, 14, 15, 17]];
        assert!(is_report_safe(&numbers[0], 2));
    }
}
