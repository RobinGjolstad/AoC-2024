use day_8::part2::process;

fn main() {
    let file = include_str!("../../input2.txt");
    let result = process(file);
    println!("{}", result);
}
