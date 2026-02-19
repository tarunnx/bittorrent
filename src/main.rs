use clap::{Parser, Subcommand};

use crate::{commands::decode::decode_bencoded_value, torrent::Torrent};

mod commands;
mod torrent;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Decode { value: String },
    Info { path: String },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Decode { value }) => {
            let ans = decode_bencoded_value(value);
            println!("ans: {}", ans);
        }
        Some(Commands::Info { path }) => {
            let f = std::fs::read(path).unwrap();
            let value: Torrent = serde_bencode::from_bytes(&f).unwrap();
            println!(
                "Tracker URL: {} Length: {}",
                value.announce, value.info.length
            )
        }
        None => {}
    }
}
