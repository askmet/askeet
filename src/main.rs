use askeet::{cli::arguments::Arguments, rsa::generate_private_key};
use clap::Parser;
use colored::Colorize;

fn main() {
    let arguments = Arguments::parse();

    match &arguments.command {
        askeet::cli::arguments::Commands::GeneratePair(args) => {
            let bits = &args.bits.unwrap_or(askeet::cli::arguments::Bits::OneK);

            let bits: usize = match bits {
                askeet::cli::arguments::Bits::OneK => 1024,
                askeet::cli::arguments::Bits::TwoK => 2048,
                askeet::cli::arguments::Bits::FourK => 4096,
            };

            println!(
                "{} : {}",
                "Selected number of bits".purple(),
                bits.to_string().bold().white()
            );

            generate_private_key(&bits);
        }
    }
}
