use std::cell::Cell;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self},
    combinator::peek,
    sequence::{delimited, separated_pair},
    IResult,
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn process(input: &str) -> usize {
    //
    let input_sum: i64 = find_all_mul_numbers(input)
        .par_iter()
        .map(|(n1, n2)| n1 * n2)
        .sum();

    input_sum
        .try_into()
        .expect("Converting from signed to unsigned should be fine.")
}

fn find_all_mul_numbers(input: &str) -> Vec<(i64, i64)> {
    let mut resulting_vec: Vec<(i64, i64)> = Vec::new();

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
                            let do_peek_res: IResult<&str, &str> = peek(take_until("do()"))(input);
                            match do_peek_res {
                                Ok(_) => {
                                    // We found do.
                                    // We can continue.
                                }
                                Err(_) => {
                                    // There were no "do()" left when "mul" is disabled, so we're done.
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

fn find_mul_numbers<'a>(input: &'a str, mul_enabled: &Cell<bool>) -> IResult<&'a str, Vec<i64>> {
    // If multiplication is disabled, scan for "do".
    // If we find "do", change value of `mul enabled` and proceed to next steps.
    //
    // If multiplication is enabled, scan for "mul" and "don't".
    // If we find "mul", proceed as normal.
    // If we find "don't", change value of `mul_enabled` and return.

    let input = if !mul_enabled.get() {
        // We must find "do" before we can do anything else.
        let (input, _captures) = take_until("do()")(input)?;

        // We found "do", so let's grab it and proceed.
        let (input, _) = tag("do()")(input)?;

        // Update the enable state
        mul_enabled.set(true);

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
    let dont_result: IResult<&str, &str> = take_until("don't()")(input);

    // If one of the results is Err, we can assume that one doesn't exist.
    let (input, _captures) = match (mul_result, dont_result) {
        (Ok((mul_input, mul_capture)), Ok((dont_input, dont_capture))) => {
            // Find the one with the shortest capture and return that one's input.
            if mul_capture.len() <= dont_capture.len() {
                // We found "mul" first, so we keep working with that.
                (mul_input, mul_capture)
            } else {
                // We found "don't" first.
                // Let's eat it.
                let (dont_input, _) = tag("don't()")(dont_input)?;
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
            // Let's eat it.
            let (dont_input, _) = tag("don't()")(dont_input)?;
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

    let (input, _) = tag("mul")(input)?;

    // Capture numbers inside ()
    let (input, inside_parens) = delimited(
        tag("("),
        separated_pair(complete::i64, complete::char(','), complete::i64),
        tag(")"),
    )(input)?;

    let numbers = vec![inside_parens.0, inside_parens.1];

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
        let input = "don't()mul(1,2)";
        let mul_enabled = Cell::new(true);
        let (_remainder, numbers) = find_mul_numbers(input, &mul_enabled).unwrap();

        assert!(numbers.is_empty());
        assert!(!mul_enabled.get());
    }

    #[test]
    fn find_mul_in_string_with_dont_trailing_finds_the_numbers() {
        let input = "mul(1,2)don't()";
        let _a = Cell::new(true);
        let (remainder, numbers) = find_mul_numbers(input, &_a).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(remainder.contains("don't()"));
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
        let input = "do()mul(1,2)";
        let mul_enabled = Cell::new(false);
        let (_remainder, numbers) = find_mul_numbers(input, &mul_enabled).unwrap();

        assert_eq!(numbers.len(), 2);
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 2);
    }

    #[test]
    fn find_mul_in_string_when_disabled_fails_if_it_is_preceded_by_dont() {
        let input = "don't()mul(1,2)";
        let mul_enabled = Cell::new(false);
        let result = find_mul_numbers(input, &mul_enabled);

        assert!(result.is_err());
    }

    #[test]
    fn find_mul_with_multiple_do_dont_over_several_lines_yields_the_correct_result() {
        let input = r#"sdfadfmul(1,2)ølajsddo()øladsf98mul(1,2)lasd
don't()asdfløjmul(1,2)ljøkadsfmul(1,2)ølahlsg
øiagsdo()p4ttqpomul(1,2)povasidon't()oøahvsdmul(1,2)"#;
        let numbers = find_all_mul_numbers(input);

        assert_eq!(numbers.len(), 3);
    }
}
