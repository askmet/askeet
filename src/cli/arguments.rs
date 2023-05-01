use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    author = "flexice",
    version = "0.1.0",
    about = "Askeet is a fast utility to encrypt text using RSA encryption",
    long_about = None
)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    GeneratePair(GeneratePairKeysArgs),
}

#[derive(Args)]
pub struct GeneratePairKeysArgs {
    #[arg(help = "Number of bits for generation key (default: 1024 bits)")]
    pub bits: Option<Bits>,
}

#[derive(ValueEnum, Clone, Copy)]
pub enum Bits {
    #[value(name = "1024")]
    OneK,
    #[value(name = "2048")]
    TwoK,
    #[value(name = "4096")]
    FourK,
}
