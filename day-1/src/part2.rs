use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn process(input: &str) -> usize {
    // Split the input into lines.
    let lines = input.lines();

    // Strip whitespace at each end.
    let lines: Vec<&str> = lines.map(|line| line.trim()).collect();

    // Split each line on whitespace, and save the two numbers in two separate lists.
    let (first_num_list, second_num_list): (Vec<usize>, Vec<usize>) = lines
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

    // For each number in the first list, find the number of times it appears in the second list.
    let num_count: Vec<(usize, usize)> = first_num_list
        .par_iter()
        .map(|num| {
            let count = second_num_list
                .par_iter()
                .filter(|second_num| *second_num == num)
                .count();

            (*num, count)
        })
        .collect();

    // Multiply the first number with the number of times it appears in the second list.
    let sum = num_count.par_iter().map(|(num, count)| num * count).sum();

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
        assert_eq!(process(input), 31);
    }
}
