use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

const SAFE_LIMIT: usize = 3;

#[derive(Debug)]
enum Direction {
    Ascending,
    Descending,
}

pub fn process(input: &str) -> usize {
    let numbers = get_numbers_from_input(input);

    let sum = numbers
        .par_iter()
        .filter_map(|nums| is_report_safe(nums).then_some(1))
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

fn is_report_safe(numbers: &[usize]) -> bool {
    let mut direction: Option<Direction> = None;

    for (a, b) in numbers.iter().zip(numbers.iter().skip(1)) {
        // Convert numbers to signed numbers.
        let (first, second): (isize, isize) = (*a as isize, *b as isize);

        // Determine the direction of the numbers.
        if direction.is_none() {
            direction = match first.cmp(&second) {
                std::cmp::Ordering::Less => Some(Direction::Ascending),
                std::cmp::Ordering::Greater => Some(Direction::Descending),
                std::cmp::Ordering::Equal => return false,
            };
        }

        // Check if the difference between the two numbers is greater than the safe limit.
        match direction {
            Some(Direction::Ascending) => {
                if second <= first {
                    // We were ascending, but the second number was smaller or equal to the first.
                    // Unsafe.
                    return false;
                }
                if isize::abs_diff(first, second) > SAFE_LIMIT {
                    // The difference between the two numbers is greater than the safe limit.
                    return false;
                }
            }
            Some(Direction::Descending) => {
                if second >= first {
                    // We were descending, but the second number was greater or equal to the first.
                    // Unsafe.
                    return false;
                }
                if isize::abs_diff(first, second) > SAFE_LIMIT {
                    // The difference between the two numbers is greater than the safe limit.
                    return false;
                }
            }
            _ => panic!("Direction was not determined."),
        }
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
        assert_eq!(process(input), 2);
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_1_2() {
        let numbers = vec![vec![1, 2]];
        assert!(is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_2_1() {
        let numbers = vec![vec![2, 1]];
        assert!(is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_1_1() {
        let numbers = vec![vec![1, 1]];
        assert!(!is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_1_5() {
        let numbers = vec![vec![1, 5]];
        assert!(!is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_5_1() {
        let numbers = vec![vec![5, 1]];
        assert!(!is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_1_4_7() {
        let numbers = vec![vec![1, 4, 7]];
        assert!(is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_true_when_passed_7_4_1() {
        let numbers = vec![vec![7, 4, 1]];
        assert!(is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_7_1_4() {
        let numbers = vec![vec![7, 1, 4]];
        assert!(!is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_1_4_1() {
        let numbers = vec![vec![1, 4, 1]];
        assert!(!is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_1_5_9() {
        let numbers = vec![vec![1, 5, 9]];
        assert!(!is_report_safe(&numbers[0]));
    }

    #[test]
    fn is_report_safe_returns_false_when_passed_9_5_1() {
        let numbers = vec![vec![9, 5, 1]];
        assert!(!is_report_safe(&numbers[0]));
    }
}
