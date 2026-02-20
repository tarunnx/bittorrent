use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TrackerResponse {
    /// An integer, indicating how often your client should make a request to the tracker.
    /// You can ignore this value for the purposes of this challenge.
    pub interval: usize,

    /// A string, which contains list of peers that your client can connect to.
    /// Each peer is represented using 6 bytes. The first 4 bytes are the peer's IP address and the last 2 bytes are the peer's port number.
    pub peers: String,
}
