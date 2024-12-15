use nom::{
    bytes::complete::{tag, take_until},
    character::complete,
    multi::separated_list1,
    IResult,
};

pub fn process(input: &str) -> usize {
    // Brute-force attempt to find combinations.
    let equations = input
        .lines()
        .filter_map(|line| {
            if let Ok((_, equation)) = parse_line(line) {
                Some(equation)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut valid_combinations = equations
        .iter()
        .flat_map(verify_equation)
        .collect::<Vec<_>>();

    valid_combinations.dedup_by(|a, b| a.result == b.result && a.numbers == b.numbers);

    valid_combinations.iter().map(|eq| eq.result as usize).sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Equation<'a> {
    result: u64,
    numbers: Vec<u64>,
    operators: Vec<&'a str>,
}

fn parse_line(input: &str) -> IResult<&str, Equation> {
    // Grab the first bit of the input, the target result.
    let (input, result) = take_until(": ")(input)?;
    let (_result_remnants, result) = complete::u64(result)?;

    // Eat the leading ": ", then start extracting numbers.
    let (input, _) = tag(": ")(input)?;

    // Extract the numbers.
    let (input, numbers) = separated_list1(tag(" "), complete::u64)(input)?;

    Ok((
        input,
        Equation {
            result,
            numbers,
            operators: Vec::new(),
        },
    ))
}

fn generate_equation_permutations<'a>(equation: &'a Equation<'a>) -> Vec<Equation<'a>> {
    // Try to find combinations of additions and multiplications which result in the target number.
    // Any combination is valid as long as it results in the target number.
    // The numbers must be in the exact order they are in the input.
    // Order of operations is always left-to-right.

    // Naive approach: Test all combinations.
    let mut combinations_to_test = Vec::new();

    // Generate all possible combinations of operators.
    // Possible operators are "+", "*", and "||" (concatenation).
    // Concatenation is used to merge two numbers together.
    // For example, 1 || 2 = 12.
    for i in 0..3_usize.pow(equation.numbers.len() as u32 - 1) {
        // Generate the operators.
        let mut operators = Vec::new();
        for j in 0..equation.numbers.len() - 1 {
            match i / 3_usize.pow(j as u32) % 3 {
                0 => operators.push("+"),
                1 => operators.push("*"),
                2 => operators.push("||"),
                _ => panic!("Invalid operator"),
            }
        }

        combinations_to_test.push(Equation {
            result: equation.result,
            numbers: equation.numbers.clone(),
            operators,
        });
    }

    combinations_to_test
}

fn verify_equation<'a>(equation: &'a Equation<'a>) -> Vec<Equation<'a>> {
    // Try to find combinations of additions and multiplications which result in the target number.
    // Any combination is valid as long as it results in the target number.
    // The numbers must be in the exact order they are in the input.
    // Order of operations is always left-to-right.

    // Naive approach: Test all combinations.
    let combinations_to_test = generate_equation_permutations(equation);

    let mut valid_combinations = Vec::new();
    for combination in combinations_to_test {
        let mut result = combination.numbers[0];
        for (i, number) in combination.numbers.iter().skip(1).enumerate() {
            match combination.operators[i] {
                "+" => result += number,
                "*" => result *= number,
                "||" => result = format!("{}{}", result, number).parse().unwrap(),
                _ => panic!("Invalid operator"),
            }
        }

        if result == combination.result {
            valid_combinations.push(combination);
        }
    }

    valid_combinations
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(process(input), 11387);
    }

    #[rstest]
    #[case("190: 10 19", Equation{result: 190, numbers: vec![10, 19], operators: vec![]})]
    #[case("3267: 81 40 27", Equation{result: 3267, numbers: vec![81, 40, 27], operators: vec![]})]
    #[case("83: 17 5", Equation{result: 83, numbers: vec![17, 5], operators: vec![]})]
    #[case("156: 15 6", Equation{result: 156, numbers: vec![15, 6], operators: vec![]})]
    #[case("7290: 6 8 6 15", Equation{result: 7290, numbers: vec![6, 8, 6, 15], operators: vec![]})]
    #[case("161011: 16 10 13", Equation{result: 161011, numbers: vec![16, 10, 13], operators: vec![]})]
    #[case("192: 17 8 14", Equation{result: 192, numbers: vec![17, 8, 14], operators: vec![]})]
    #[case("21037: 9 7 18 13", Equation{result: 21037, numbers: vec![9, 7, 18, 13], operators: vec![]})]
    #[case("292: 11 6 16 20", Equation{result: 292, numbers: vec![11, 6, 16, 20], operators: vec![]})]
    fn test_parse_equations(#[case] input: &str, #[case] expected: Equation) {
        let (_input, result) = parse_line(input).unwrap();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(
        Equation{result: 190, numbers: vec![10, 19], operators: vec![]},
        vec![
            Equation{result: 190, numbers: vec![10, 19], operators: vec!["*"]}
        ]
    )]
    #[case(
        Equation{result: 3267, numbers: vec![81, 40, 27], operators: vec![]},
        vec![
            Equation{result: 3267, numbers: vec![81, 40, 27], operators: vec!["*","+"]},
            Equation{result: 3267, numbers: vec![81, 40, 27], operators: vec!["+","*"]},
        ]
    )]
    #[case(
        Equation{result: 83, numbers: vec![17, 5], operators: vec![]},
        vec![
        ]
    )]
    #[case(
        Equation{result: 156, numbers: vec![15, 6], operators: vec![]},
        vec![
            Equation{result: 156, numbers: vec![15, 6], operators: vec!["||"]}
        ]
    )]
    #[case(
        Equation{result: 7290, numbers: vec![6, 8, 6, 15], operators: vec![]},
        vec![
            Equation{result: 7290, numbers: vec![6, 8, 6, 15], operators: vec!["*","||","*"]},
        ]
    )]
    #[case(
        Equation{result: 161011, numbers: vec![16, 10, 13], operators: vec![]},
        vec![
        ]
    )]
    #[case(
        Equation{result: 192, numbers: vec![17, 8, 14], operators: vec![]},
        vec![
            Equation{result: 192, numbers: vec![17, 8, 14], operators: vec!["||","+"]}
        ]
    )]
    #[case(
        Equation{result: 21037, numbers: vec![9, 7, 18, 13], operators: vec![]},
        vec![
        ]
    )]
    #[case(
        Equation{result: 292, numbers: vec![11, 6, 16, 20], operators: vec![]},
        vec![
            Equation{result: 292, numbers: vec![11, 6, 16, 20], operators: vec!["+","*","+"]},
        ]
    )]
    fn test_verify_equation(#[case] input: Equation, #[case] expected: Vec<Equation>) {
        assert_eq!(verify_equation(&input), expected);
    }
}

