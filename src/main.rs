mod day_1;
mod day_2;
mod day_3;
mod day_5;

use std::fs;

use clap::{Parser, ValueEnum};
use day_1::{calculate_distance, calculate_simularity_score};
use day_2::check_levels_safe;
use day_3::{decode_memory, decode_memory_full};
use day_5::{check_puzzle_5, check_puzzle_5_part_2};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Puzzle {
    Day1Puzzle1,
    Day1Puzzle2,
    Day2Puzzle1,
    Day2Puzzle2,
    Day3Puzzle1,
    Day3Puzzle2,
    Day5Puzzle1,
    Day5Puzzle2,
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
        Puzzle::Day1Puzzle2 => {
            let result = calculate_simularity_score(contents);
            println!("Simularity Score is {}", result)
        }
        Puzzle::Day2Puzzle1 => {
            let result = check_levels_safe(contents, false);
            println!("Num safe reports {}", result)
        }
        Puzzle::Day2Puzzle2 => {
            let result = check_levels_safe(contents, true);
            println!("Num safe reports {}", result)
        }
        Puzzle::Day3Puzzle1 => {
            let result = decode_memory(contents);
            println!("Memory decoded {}", result)
        }
        Puzzle::Day3Puzzle2 => {
            let result = decode_memory_full(contents);
            println!("Memory decoded {}", result)
        }
        Puzzle::Day5Puzzle1 => {
            let result = check_puzzle_5(contents);
            println!("Total {}", result)
        }
        Puzzle::Day5Puzzle2 => {
            let result = check_puzzle_5_part_2(contents);
            println!("Total {}", result)
        }
    }
}
