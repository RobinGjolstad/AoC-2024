use day_8::part1::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{}", result);
}
