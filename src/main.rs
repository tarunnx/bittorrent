use std::path::PathBuf;

use anyhow::Context;
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
    Info { torrent: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match &args.command {
        Some(Commands::Decode { value }) => {
            let ans = decode_bencoded_value(value);
            println!("ans: {}", ans);
            Ok(())
        }
        Some(Commands::Info { torrent }) => {
            let f = std::fs::read(torrent).context("read torrent file")?;
            let value: Torrent = serde_bencode::from_bytes(&f).context("parse torrent file")?;
            println!(
                "Tracker URL: {} Length: {}",
                value.announce, value.info.piece_len
            );
            Ok(())
        }
        _ => anyhow::bail!("invalid command"),
    }
}
