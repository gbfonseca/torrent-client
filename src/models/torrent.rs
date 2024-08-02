use serde_bytes::ByteBuf;
use serde_derive::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Node(String, i64);

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub path: Vec<String>,
    pub length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pieces: ByteBuf,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    #[serde(default)]
    pub md5sum: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
    #[serde(default)]
    pub files: Option<Vec<File>>,
    #[serde(default)]
    pub private: Option<u8>,
    #[serde(default)]
    pub path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    pub root_hash: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Torrent {
    pub info: Info,
    #[serde(default)]
    pub announce: Option<String>,
    #[serde(default)]
    pub nodes: Option<Vec<Node>>,
    #[serde(default)]
    pub encoding: Option<String>,
    #[serde(default)]
    pub httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    pub announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    pub creation_date: Option<i64>,
    #[serde(rename = "comment")]
    pub comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    pub created_by: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct TorrentFile {
    pub announce: String,

    pub info_hash: Vec<u8>,

    pub piece_hashes: Vec<Vec<u8>>,

    pub piece_length: i64,

    pub name: String,
}

#[allow(dead_code)]
impl Torrent {
    pub fn to_torrent_file(&self) -> TorrentFile {
        let mut hasher = Sha1::new();
        hasher.update(format!("{:?}", self.info));
        let info_hash = format!("{:x}", hasher.finalize());

        TorrentFile {
            announce: self.announce.clone().unwrap_or("".into()),
            name: self.info.name.clone(),
            piece_length: self.info.piece_length,
            info_hash: info_hash.as_bytes().into(),
            piece_hashes: vec![vec![]],
        }
    }
}
