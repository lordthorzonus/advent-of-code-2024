mod days;
mod utils;

use std::error::Error;
use clap::{Args, Parser, Subcommand};
use crate::days::{Day, DaySolver};
use crate::utils::read_input_file;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch a specific day's data
    Download(DownloadArgs),

    /// Solve a problem using input file
    Solve(SolveArgs),
}

#[derive(Args)]
struct DownloadArgs {
    /// Day number to download (1-25)
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day_number: u8,
}

#[derive(Args)]
struct SolveArgs {
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day_number: u8,
    /// Input file path for solving the problem
    #[arg(short, long)]
    input_file_path: String,
}


struct Solution(
    String,
    String,
);

fn execute_solve(args: &SolveArgs) -> Result<Solution, Box<dyn Error>> {
    let input = read_input_file(&args.input_file_path)?;
    let day: Day = args.day_number.try_into()?;
    let solver: Box<dyn DaySolver> = day.try_into()?;

    Ok(Solution(solver.solve_part1(&input)?, solver.solve_part2(&input)?))
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Download(_args) => {
            todo!()
        }
        Commands::Solve(args) => {
            execute_solve(args)
        }
    };

    match result {
        Err(e) => {
            println!("{}", e)
        }
        Ok(solution) => {
            println!("Part1: {} \n\nPart2: {}", solution.0, solution.1)
        }
    }

}
