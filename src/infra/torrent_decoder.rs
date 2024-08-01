use std::fs;

use serde_bencode::value::Value;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Torrent {
    #[serde(rename = "url-list")]
    pub url_list: Vec<String>,
    #[serde(rename = "info")]
    pub info: Value
}

pub struct TorrentDecoder {}

impl TorrentDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parser(&self) {
        let content = fs::read("archlinux-2024.07.01-x86_64.iso.torrent").unwrap();
      
    //   println!("{}", String::from_utf8_lossy(&content.clone()));
      
        let torrent: Torrent = serde_bencode::from_bytes(&content).unwrap();

        println!("{:?}", torrent.info);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_decode_torrent_file() {
        let torrent_decoder = TorrentDecoder::new();
torrent_decoder.parser();
        assert_eq!(1, 2)
    }
}