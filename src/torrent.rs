#![allow(dead_code)]

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    /// tracker URL
    pub announce: String,
    /// describes the file
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    /// suggested filename/directory name
    pub name: String,
    /// bytes per piece
    #[serde(rename = "piece length")]
    pub piece_len: usize,
    /// concatenated SHA1 hashes
    #[serde(with = "serde_bytes")]
    pub pieces: Vec<u8>,
    pub length: usize,
    pub files: Option<Vec<File>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct File {
    pub length: usize,
    pub path: Vec<String>,
}
