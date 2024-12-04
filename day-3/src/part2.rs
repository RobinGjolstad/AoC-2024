use std::cell::Cell;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until},
    character::{
        complete::{alphanumeric1, anychar, digit1},
        is_alphanumeric, is_digit,
    },
    combinator::peek,
    error::Error,
    multi::{many0, many1, many_till},
    sequence::{delimited, separated_pair},
    IResult,
};
use rayon::{
    prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

pub fn process(input: &str) -> usize {
    // Iterate over all lines in file.
    let lines: Vec<&str> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.trim())
            }
        })
        .collect();
    let sum: isize = lines
        .par_iter()
        .enumerate()
        .map(|(index, line)| {
            println!("Parsing numbers for line: {index}");
            // Find all mul-pairs
            let num_pair_list = find_all_mul_numbers(line);
            let line_sum = num_pair_list
                .par_iter()
                .map(|(n1, n2)| {
                    // Multiply the numbers
                    n1 * n2
                })
                .sum::<isize>();

            println!("Sum for line {index} is: {line_sum}");
            line_sum
        })
        .sum();

    sum.try_into()
        .expect("Converting from signed to unsigned should be fine.")
}

fn find_all_mul_numbers(input: &str) -> Vec<(isize, isize)> {
    let mut resulting_vec: Vec<(isize, isize)> = Vec::new();

    let mul_enabled: Cell<bool> = Cell::new(true);

    // Initial test before we start spinning.
    // Ugly hack for enable to bypass borrow-checker nonsense.
    let mut result = find_mul_numbers(input, &mul_enabled);

    loop {
        match result {
            Ok((input, numbers)) => {
                // Either we found a `mul(x,y)` or we caught nothing since we were "disabled".
                match numbers.len() {
                    2 => {
                        // All good, we found `mul(x,y)`
                        let numbers = (numbers[0], numbers[1]);
                        resulting_vec.push(numbers);
                    }
                    0 => (), // We were probably disabled. Try again.
                    _ => panic!("We somehow got neither 0 nor 2 numbers."),
                }

                if input.is_empty() {
                    // We've fully parsed the string.
                    break;
                }

                // "start" a new test.
                result = find_mul_numbers(input, &mul_enabled)
            }

            Err(ref e) => {
                match e {
                    nom::Err::Error(e) => {
                        // Failed parsing, probably a false positive of some sort.
                        // Might have consumed a `mul` but found incorrect info after this.
                        // Retry from current position.
                        let (input, _error_type) = (e.input, e.code);

                        // Check if we:
                        // - If mul enabled: look for next mul.
                        // - If mul disabled, look for "do". Look out for possible false positives!
                        if mul_enabled.get() {
                            // Look for mul.
                            // If there are none left, stop.
                            let peek_res: IResult<&str, &str> = peek(take_until("mul"))(input);
                            if peek_res.is_err() {
                                println!("There are no more `mul`s left.");
                                break;
                            }
                        } else {
                            // Look for "do".
                            // Beware of false positive from "don't".
                            let do_peek_res: IResult<&str, &str> = peek(take_until("do"))(input);
                            let dont_peek_res: IResult<&str, &str> =
                                peek(take_until("don't"))(input);
                            let input: &str = match (do_peek_res, dont_peek_res) {
                                (Ok((do_input, do_consumed)), Ok((dont_input, dont_consumed))) => {
                                    // We found both.
                                    // Were they the same?
                                    if do_consumed == dont_consumed {
                                        // It was a false positive. we only found a "don't"
                                        // Consume the "don't" and try again.
                                        todo!()
                                    } else {
                                        // Mismatch.
                                        // This can only occur if the "do" was found first.
                                        // We can proceed.
                                        todo!()
                                    }
                                }
                                (Ok(_), Err(_)) => {
                                    // We found a "do", but no "don't".
                                    // We can proceed.
                                    todo!()
                                }
                                _ => {
                                    // We found neither, or some weird combination.
                                    // Nothing left to do.
                                    break;
                                }
                            };
                        }

                        result = find_mul_numbers(input, &mul_enabled);
                    }
                    nom::Err::Incomplete(needed) => {
                        // Not enough data.
                        eprintln!("Failed parsing data. Not enough data left to match.\nWould need {:?} more characters to complete.", needed);
                        break;
                    }
                    nom::Err::Failure(e) => {
                        // Unrecoverable error.
                        eprintln!("Unrecoverable parsing error: {}", e);
                        break;
                    }
                }
            }
        }
    }

    resulting_vec
}

fn find_mul_numbers<'a>(input: &'a str, mul_enabled: &Cell<bool>) -> IResult<&'a str, Vec<isize>> {
    // If multiplication is disabled, scan for "do".
    // If we find "do", change value of `mul enabled` and proceed to next steps.
    //
    // If multiplication is enabled, scan for "mul" and "don't".
    // If we find "mul", proceed as normal.
    // If we find "don't", change value of `mul_enabled` and return.

    let input = if !mul_enabled.get() {
        // We must find "do" before we can do anything else.
        let (input, _captures) = take_until("do")(input)?;

        // Check if it's a false positive for "don't".
        let peek_res: IResult<&str, &str> = peek(tag("don't"))(input);
        match peek_res {
            Ok(_) => {
                // This was a false positive for a "do".
                // Let's eat the "don't" and return empty.
                let (input, _) = tag("don't")(input)?;
                return Ok((input, vec![]));
            }
            Err(_) => {
                // This was a real "do".
                // Carry on.
            }
        }

        // We found "do", so let's grab it and proceed.
        let (input, _) = tag("do")(input)?;

        // Now we can proceed.
        input
    } else {
        input
    };

    // Try to look for both "mul" and "don't".
    // If both exist, we should continue from the shortest "capture".
    // If only one exists, that's the one we'll work with.
    // If neither exist, propagate an error, preferrably the one with the longest remaining input.
    let mul_result: IResult<&str, &str> = take_until("mul")(input);
    let dont_result: IResult<&str, &str> = take_until("don't")(input);

    // If one of the results is Err, we can assume that one doesn't exist.
    let (input, captures) = match (mul_result, dont_result) {
        (Ok((mul_input, mul_capture)), Ok((dont_input, dont_capture))) => {
            // Find the one with the shortest capture and return that one's input.
            if mul_capture.len() <= dont_capture.len() {
                // We found "mul" first, so we keep working with that.
                (mul_input, mul_capture)
            } else {
                // We found "don't" first.
                // Update `mul_enabled` and return.
                mul_enabled.set(false);
                return Ok((dont_input, vec![]));
            }
        }
        (Ok((mul_input, mul_capture)), Err(_)) => {
            // We found "mul" and not "don't". All good!
            (mul_input, mul_capture)
        }
        (Err(_), Ok((dont_input, _dont_capture))) => {
            // We found "don't".
            // Update `mul_enabled` and return.
            mul_enabled.set(false);
            return Ok((dont_input, vec![]));
        }
        (Err(mul_err), Err(_)) => {
            // Neither "mul" nor "don't" were found.
            // Return "mul"s error.
            return Err(mul_err);
        }
    };

    //dbg!(&input, &captures);

    let (input, _) = tag("mul")(input)?;
    // dbg!(&input);

    // Capture numbers inside ()
    let (input, inside_parens) = delimited(
        tag("("),
        many1(nom::branch::alt((tag("-"), tag(","), alphanumeric1))),
        tag(")"),
    )(input)?;
    // dbg!(&input, &inside_parens);

    // Parse as isize
    let numbers: Vec<isize> = inside_parens
        .par_iter()
        .filter_map(|str| str.parse::<isize>().ok())
        .collect();

    Ok((input, numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(process(input), 48);
    }

    #[test]
    fn find_mul_in_a_string_with_only_a_valid_option() {
        let input = "mul(1,2)";
        let _a = Cell::new(true);
        let (remainder, numbers) = find_mul_numbers(input, &_a).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(remainder.is_empty());
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 2);
    }

    #[test]
    fn find_mul_in_a_string_preceded_by_noise() {
        let input = "abc123noisemul(3,4)";
        let _a = Cell::new(true);
        let (remainder, numbers) = find_mul_numbers(input, &_a).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(remainder.is_empty());
        assert_eq!(numbers[0], 3);
        assert_eq!(numbers[1], 4);
    }

    #[test]
    fn find_mul_in_a_string_with_noise_on_either_side() {
        let input = "abc123noisemul(5,6)noise123abc";
        let _a = Cell::new(true);
        let (remainder, numbers) = find_mul_numbers(input, &_a).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(!remainder.is_empty());
        assert_eq!(numbers[0], 5);
        assert_eq!(numbers[1], 6);
    }

    #[test]
    fn find_several_mul_in_string_finds_all() {
        let input = "abcmul(1,2)mul(3,4)defmul(5,6)ghimul[7,8]";
        let _a = Cell::new(true);
        let (remainder0, numbers0) =
            find_mul_numbers(input, &_a).expect("The first mul should be found.");
        assert_eq!((numbers0[0], numbers0[1]), (1, 2));

        let _a = Cell::new(true);
        let (remainder1, numbers1) =
            find_mul_numbers(remainder0, &_a).expect("The second mul should be found.");
        assert_eq!((numbers1[0], numbers1[1]), (3, 4));

        let _a = Cell::new(true);
        let (remainder2, numbers2) =
            find_mul_numbers(remainder1, &_a).expect("The third mul should be found");
        assert_eq!((numbers2[0], numbers2[1]), (5, 6));

        let _a = Cell::new(true);
        let should_error = find_mul_numbers(remainder2, &_a);
        assert!(should_error.is_err());
    }

    #[test]
    fn find_all_mul_numbers_finds_them_all() {
        let input = "abcmul(1,2)abcmul[42,69]abcmul(3,4)defmul(5,6)ghimul[7,8]";
        let num_vec = find_all_mul_numbers(input);

        assert_eq!(num_vec.len(), 3);
        assert_eq!(num_vec[0], (1, 2));
        assert_eq!(num_vec[1], (3, 4));
        assert_eq!(num_vec[2], (5, 6));
    }

    #[test]
    fn find_mul_in_string_starting_with_dont_finds_no_numbers() {
        let input = "don'tmul(1,2)";
        let mul_enabled = Cell::new(true);
        let (_remainder, numbers) = find_mul_numbers(input, &mul_enabled).unwrap();

        assert!(numbers.is_empty());
        assert!(!mul_enabled.get());
    }

    #[test]
    fn find_mul_in_string_with_dont_trailing_finds_the_numbers() {
        let input = "mul(1,2)don't";
        let _a = Cell::new(true);
        let (remainder, numbers) = find_mul_numbers(input, &_a).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(remainder.contains("don't"));
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 2);
    }

    #[test]
    fn find_mul_in_string_when_disabled_fails() {
        let input = "mul(1,2)";
        let _a = Cell::new(false);
        let res = find_mul_numbers(input, &_a);

        assert!(res.is_err());
    }

    #[test]
    fn find_mul_in_string_when_disabled_works_if_it_is_preceded_by_do() {
        let input = "domul(1,2)";
        let mul_enabled = Cell::new(false);
        let (_remainder, numbers) = find_mul_numbers(input, &mul_enabled).unwrap();

        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 2);
    }

    #[test]
    fn find_mul_in_string_when_disabled_fails_if_it_is_preceded_by_dont() {
        let input = "don'tmul(1,2)";
        let mul_enabled = Cell::new(false);
        let (remainder, numbers) = find_mul_numbers(input, &mul_enabled).unwrap();

        assert!(numbers.is_empty());
        assert!(remainder.contains("mul(1,2)"));
    }
}

