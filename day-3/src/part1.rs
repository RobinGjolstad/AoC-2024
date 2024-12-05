use nom::{
    bytes::complete::{tag, take_until},
    character::complete::alphanumeric1,
    multi::many1,
    sequence::delimited,
    IResult,
};
use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    str::ParallelString,
};

pub fn process(input: &str) -> usize {
    // Iterate over all lines in file.
    let sum: isize = input
        .par_lines()
        .map(|line| {
            // Find all mul-pairs
            let num_pair_list = find_all_mul_numbers(line);
            let line_sum = num_pair_list
                .par_iter()
                .map(|(n1, n2)| {
                    // Multiply the numbers
                    n1 * n2
                })
                .sum::<isize>();

            line_sum
        })
        .sum();

    sum.try_into()
        .expect("Converting from signed to unsigned should be fine.")
}

fn find_all_mul_numbers(input: &str) -> Vec<(isize, isize)> {
    let mut resulting_vec: Vec<(isize, isize)> = Vec::new();

    // Initial test before we start spinning.
    let mut result = find_mul_numbers(input);

    loop {
        match result {
            Ok((input, numbers)) => {
                // We found a `mul(x,y)`
                assert_eq!(
                    numbers.len(),
                    2,
                    "`find_mul_numbers` should only find pairs of numbers."
                );
                let numbers = (numbers[0], numbers[1]);
                resulting_vec.push(numbers);

                if input.is_empty() {
                    // We've fully parsed the string.
                    break;
                }

                // "start" a new test.
                result = find_mul_numbers(input)
            }

            Err(ref e) => match e {
                nom::Err::Error(e) => {
                    // Failed parsing, probably a false positive of some sort.
                    // Might have consumed a `mul` but found incorrect info after this.
                    let (input, _error_type) = (e.input, e.code);

                    // Do a peek at the remaining input and look for `mul`.
                    // If there are none left, stop.
                    let peek_res: IResult<&str, &str> =
                        nom::combinator::peek(take_until("mul"))(input);
                    if peek_res.is_err() {
                        println!("There are no more `mul`s left.");
                        break;
                    }

                    // There are still `mul`s left.
                    // Retry from current position.
                    result = find_mul_numbers(input);
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
            },
        }
    }

    resulting_vec
}

fn find_mul_numbers(input: &str) -> IResult<&str, Vec<isize>> {
    // Optionally skip until "mul"
    // `captures` contains anything _before_ the tag, if anything.
    let (input, _captures) = take_until("mul")(input)?;

    let (input, _) = tag("mul")(input)?;

    // Capture numbers inside ()
    let (input, inside_parens) = delimited(
        tag("("),
        many1(nom::branch::alt((tag("-"), tag(","), alphanumeric1))),
        tag(")"),
    )(input)?;

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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process(input), 161);
    }

    #[test]
    fn find_mul_in_a_string_with_only_a_valid_option() {
        let input = "mul(1,2)";
        let (remainder, numbers) = find_mul_numbers(input).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(remainder.is_empty());
        assert_eq!(numbers[0], 1);
        assert_eq!(numbers[1], 2);
    }

    #[test]
    fn find_mul_in_a_string_preceded_by_noise() {
        let input = "abc123noisemul(3,4)";
        let (remainder, numbers) = find_mul_numbers(input).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(remainder.is_empty());
        assert_eq!(numbers[0], 3);
        assert_eq!(numbers[1], 4);
    }

    #[test]
    fn find_mul_in_a_string_with_noise_on_either_side() {
        let input = "abc123noisemul(5,6)noise123abc";
        let (remainder, numbers) = find_mul_numbers(input).unwrap();

        assert_eq!(numbers.len(), 2);
        assert!(!remainder.is_empty());
        assert_eq!(numbers[0], 5);
        assert_eq!(numbers[1], 6);
    }

    #[test]
    fn find_several_mul_in_string_finds_all() {
        let input = "abcmul(1,2)mul(3,4)defmul(5,6)ghimul[7,8]";
        let (remainder0, numbers0) =
            find_mul_numbers(input).expect("The first mul should be found.");
        assert_eq!((numbers0[0], numbers0[1]), (1, 2));

        let (remainder1, numbers1) =
            find_mul_numbers(remainder0).expect("The second mul should be found.");
        assert_eq!((numbers1[0], numbers1[1]), (3, 4));

        let (remainder2, numbers2) =
            find_mul_numbers(remainder1).expect("The third mul should be found");
        assert_eq!((numbers2[0], numbers2[1]), (5, 6));

        let should_error = find_mul_numbers(remainder2);
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
}
