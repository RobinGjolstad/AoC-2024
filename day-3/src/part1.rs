use nom::{
    bytes::complete::{tag, take_till, take_until},
    character::{
        complete::{alphanumeric1, anychar, digit1},
        is_alphanumeric, is_digit,
    },
    error::Error,
    multi::{many0, many1, many_till},
    sequence::{delimited, separated_pair},
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
            dbg!(&num_pair_list);
            let line_sum = num_pair_list
                .par_iter()
                .map(|(n1, n2)| {
                    // Multiply the numbers
                    println!("{n1} x {n2}");
                    n1 * n2
                })
                .sum::<isize>();
            dbg!(&line_sum);

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

                // "start" a new test.
                result = find_mul_numbers(input)
            }

            Err(e) => match e {
                nom::Err::Error(e) => todo!(),
                _ => {
                    // Unrecoverable errors.
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
    let (input, captures) = take_until("mul")(input)?;
    dbg!(&input, &captures);

    let (input, _) = tag("mul")(input)?;
    dbg!(&input);

    // Capture numbers inside ()
    let (input, inside_parens) = delimited(
        tag("("),
        many1(nom::branch::alt((tag("-"), tag(","), alphanumeric1))),
        tag(")"),
    )(input)?;
    dbg!(&input, &inside_parens);

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