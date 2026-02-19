#![allow(dead_code)]

/*
* Info Hash

  The entire info dictionary is SHA-1 hashed into a single 20-byte
  value called the info hash. This is the unique identifier for a
  torrent. When peers talk to each other, they use this hash to say
 "I want THIS specific torrent."

 How Downloading Works

   1. You parse the .torrent file — get tracker URL and info hash
   2. You contact the tracker — send it your info hash, your
   IP/port, and say "I want peers"
   3. Tracker responds with a list of peers (IP:port pairs) who have
    the file
   4. You connect to peers via TCP and do a handshake — exchange
   info hashes to confirm you're both talking about the same torrent
   5. Peers tell you what pieces they have — via a "bitfield"
   message (a bit array where each bit = one piece)
   6. You request pieces — typically using a strategy like "rarest
   first" (download the least common piece first, so it doesn't
   disappear if a peer leaves)
   7. You verify each piece — hash it, compare to the expected hash
   8. You start sharing — as soon as you have a complete piece,
   other peers can download it from you
*
*/

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    /// tracker URL
    pub announce: String,
    /// describes the file
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    /// suggested filename/directory name - The name key maps to a UTF-8 encoded string
    /// which is the suggested name to save the file (or directory) as.
    pub name: String,

    /// bytes per piece - piece length maps to the number of bytes in each piece the file
    /// is split into. For the purposes of transfer, files are split into fixed-size pieces
    /// which are all the same length except for possibly the last one which may be truncated.
    /// piece length is almost always a power of two,
    /// most commonly 2 18 = 256 K (BitTorrent prior to version 3.2 uses 2 20 = 1 M as default).
    #[serde(rename = "piece length")]
    pub piece_len: usize,

    /// concatenated SHA1 hashes - pieces maps to a string whose length is a multiple of 20.
    /// It is to be subdivided into strings of length 20, each of which is the SHA1 hash of
    /// the piece at the corresponding index.
    pub pieces: Pieces,

    /// If length is present then the download represents a single file
    /// In the single file case, length maps to the length of the file in bytes
    #[serde(flatten)]
    keys: Keys,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Keys {
    SingleFile {
        length: usize,
    },
    MultiFile {
        /*
        * example -
        * Say you're sharing a music album folder structured like this:

          my-album/
          ├── disc1/
          │   ├── track01.mp3
          │   └── track02.mp3
          └── cover.jpg

          then files would be :-
        * files: [
              { length: 5000000, path: ["disc1", "track01.mp3"] },
              { length: 4200000, path: ["disc1", "track02.mp3"] },
              { length: 150000,  path: ["cover.jpg"] },
          ]
        */
        files: Vec<File>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    /// The length of the file, in bytes
    pub length: usize,

    /// A list of UTF-8 encoded strings corresponding to subdirectory names,
    /// the last of which is the actual file name (a zero length list is an error case).
    pub path: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Pieces(Vec<[u8; 20]>);
struct PiecesVisitor;

impl<'de> Visitor<'de> for PiecesVisitor {
    type Value = Pieces;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("byte string whose length is a multiple of 20")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.len() % 20 != 0 {
            return Err(E::custom(format!("length is {}", v.len())));
        }
        let mut values = Vec::with_capacity(v.len() / 20);

        for chunk in v.chunks_exact(20) {
            values.push(chunk.try_into().expect("guaranteed to be length 20"));
        }

        Ok(Pieces(values))
    }
}

impl<'de> Deserialize<'de> for Pieces {
    fn deserialize<D>(deserializer: D) -> Result<Pieces, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(PiecesVisitor)
    }
}
