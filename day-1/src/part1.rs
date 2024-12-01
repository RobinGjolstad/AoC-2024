use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn process(input: &str) -> usize {
    // Split the input into lines.
    let lines = input.lines();

    // Strip whitespace at each end.
    let lines: Vec<&str> = lines.map(|line| line.trim()).collect();

    // Split each line on whitespace, and save the two numbers in two separate lists.
    let (mut first_num_list, mut second_num_list): (Vec<usize>, Vec<usize>) = lines
        .par_iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let (first_num_str, second_num_str) = parts.split_at(1);

            (
                first_num_str.first().unwrap().parse::<usize>().unwrap(),
                second_num_str.first().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip();

    // Sort each list in ascending order.
    first_num_list.sort();
    second_num_list.sort();

    // Calculate the sum of differences between the numbers.
    let sum = first_num_list
        .iter()
        .zip(second_num_list)
        .map(|(first, second)| first.abs_diff(second))
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = r#"3   4
            4   3
            2   5
            1   3
            3   9
            3   3"#;
        dbg!(&input);
        assert_eq!(process(input), 11);
    }
}
