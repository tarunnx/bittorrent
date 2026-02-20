use bittorrent::{
    commands::decode::decode_bencoded_value,
    torrent::{Keys, Torrent},
    tracker::request::TrackerRequest,
};
use sha1::{Digest, Sha1};
use std::path::PathBuf;

use anyhow::Context;
use clap::{Parser, Subcommand};

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
    Peers { torrent: PathBuf },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

            println!("Info Hash: {}", hex::encode(result));
            println!("Piece Length: {}", value.info.piece_len);

            println!("Pieces Hashes: ");
            for piece in value.info.pieces.0 {
                println!("{}", hex::encode(piece));
            }

            Ok(())
        }
        Some(Commands::Peers { torrent }) => {
            let f = std::fs::read(torrent).context("read torrent file")?;
            let value: Torrent = serde_bencode::from_bytes(&f).context("parse torrent file")?;

            let bencoded_info =
                serde_bencode::to_bytes(&value.info).context("bencoding of info")?;
            let mut hasher = Sha1::new();

            hasher.update(bencoded_info);
            let info_hash = hasher.finalize();

            let length = if let Keys::SingleFile { length } = value.info.keys {
                length
            } else {
                todo!()
            };

            // let request = TrackerRequest {
            //     info_hash,
            //     compact: 1,
            //     uploaded: 0,
            //     downloaded: 0,
            //     left: length,
            //     peer_id: String::from("00112233445566778899"),
            //     port: 6881,
            // };
            Ok(())
        }
        _ => anyhow::bail!("invalid command"),
    }
}
