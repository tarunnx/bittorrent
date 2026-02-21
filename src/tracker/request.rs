use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct TrackerRequest {
    /// the info hash of the torrent
    ///
    /// 20 bytes long, will need to be URL encoded
    /// Note: this is NOT the hexadecimal representation, which is 40 bytes long
    /// not included because of serde_urlencoder was not working for this
    // pub info_hash: [u8; 20],

    /// a unique identifier for your client
    /// A string of length 20 that you get to pick.
    pub peer_id: String,

    /// the port your client is listening on
    /// You can set this to 6881, you will not have to support this functionality during this challenge.
    pub port: u16,

    /// the total amount uploaded so far
    /// Since your client hasn't uploaded anything yet, you can set this to 0.
    pub uploaded: usize,

    /// the total amount downloaded so far
    /// Since your client hasn't downloaded anything yet, you can set this to 0.
    pub downloaded: usize,

    /// the number of bytes left to download
    /// Since you client hasn't downloaded anything yet, this'll be the total length of the file (you've extracted this value from the torrent file in previous stages)
    pub left: usize,

    /// whether the peer list should use the compact representation
    ///
    /// For the purposes of this challenge, set this to 1.
    /// The compact representation is more commonly used in the wild, the non-compact representation is mostly supported for backward-compatibility.
    pub compact: usize,
}

pub fn urlencode(t: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * t.len());

    for &byte in t {
        encoded.push('%');
        encoded.push_str(&hex::encode(&[byte]));
    }

    encoded
}
