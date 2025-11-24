use std::{fs, path::PathBuf};

use clap::Parser;
use day_1::{part1, part2};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let file_path: String = fs::read_to_string(args.path).unwrap();
    let result1 = part1::process(file_path.as_str());
    let result2 = part2::process(file_path.as_str());
    println!("part1: {}", result1);
    println!("part2: {}", result2);
}
