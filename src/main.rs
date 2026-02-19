use clap::{Parser, Subcommand};

use crate::commands::decode::decode_bencoded_value;

mod commands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Decode { value: String },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Decode { value }) => {
            let ans = decode_bencoded_value(value.as_str());
            println!("{}", ans);
        }
        None => {}
    }
}
