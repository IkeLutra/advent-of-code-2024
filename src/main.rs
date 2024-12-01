mod day_1;

use std::fs;

use clap::{Parser, ValueEnum};
use day_1::calculate_distance;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Puzzle {
    Day1Puzzle1,
}

#[derive(Parser)]
struct Cli {
    #[arg(value_enum)]
    puzzle: Puzzle,
    /// The path to the file to read
    input: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let contents = fs::read_to_string(cli.input).expect("Should have been able to read the file");
    match cli.puzzle {
        Puzzle::Day1Puzzle1 => {
            let result = calculate_distance(contents);
            println!("Distance is {}", result)
        }
    }
}
