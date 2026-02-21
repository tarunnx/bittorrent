use bittorrent::{
    commands::decode::decode_bencoded_value,
    torrent::{Keys, Torrent},
    tracker::request::TrackerRequest,
};
use std::path::PathBuf;

use anyhow::{Context, Ok};
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
            let t: Torrent = serde_bencode::from_bytes(&f).context("parse torrent file")?;
            println!("Tracker URL: {}", t.announce);

            if let Keys::SingleFile { length } = t.info.keys {
                println!("Length: {}", length);
            } else {
                todo!()
            }

            let info_hash = t.info_hash().context("info hash calculation")?;

            println!("Info Hash: {}", hex::encode(info_hash));
            println!("Piece Length: {}", t.info.piece_len);

            println!("Piece Hashes: ");
            for piece in t.info.pieces.0 {
                println!("{}", hex::encode(piece));
            }

            Ok(())
        }
        Some(Commands::Peers { torrent }) => {
            let f = std::fs::read(torrent).context("read torrent file")?;
            let t: Torrent = serde_bencode::from_bytes(&f).context("parse torrent file")?;

            let info_hash = t.info_hash().context("calculation of info hash")?;

            let length = if let Keys::SingleFile { length } = t.info.keys {
                length
            } else {
                todo!()
            };

            let request = TrackerRequest {
                info_hash: info_hash,
                compact: 1,
                uploaded: 0,
                downloaded: 0,
                left: length,
                peer_id: String::from("00112233445566778899"),
                port: 6881,
            };

            let mut tracker_url =
                reqwest::Url::from(t.announce.parse().context("String to reqwest Url")?);
            let url_params = serde_urlencoded::to_string(&request).context("url encoding")?;

            tracker_url.set_query(Some(&url_params));

            let response = reqwest::get(tracker_url).await;

            println!("{:?}", response);

            Ok(())
        }
        _ => anyhow::bail!("invalid command"),
    }
}
