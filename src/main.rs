use sha1::{Digest, Sha1};
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};

use crate::{
    commands::decode::decode_bencoded_value,
    torrent::{Keys, Torrent},
};

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
            println!("{}", ans);
            Ok(())
        }
        Some(Commands::Info { torrent }) => {
            let f = std::fs::read(torrent).context("read torrent file")?;
            let value: Torrent = serde_bencode::from_bytes(&f).context("parse torrent file")?;
            println!("Tracker URL: {}", value.announce);

            if let Keys::SingleFile { length } = value.info.keys {
                println!("Length: {}", length);
            } else {
                todo!()
            }

            let bencoded_info =
                serde_bencode::to_bytes(&value.info).context("bencoding of info")?;
            let mut hasher = Sha1::new();

            // process input message
            hasher.update(bencoded_info);
            let result = hasher.finalize();

            println!("Info Hash: {:02x}", result);

            Ok(())
        }
        _ => anyhow::bail!("invalid command"),
    }
}
