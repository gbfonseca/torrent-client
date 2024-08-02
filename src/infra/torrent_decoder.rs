use std::{fs, path::PathBuf};

use crate::models::torrent::{Torrent, TorrentFile};

#[allow(dead_code)]
pub struct TorrentDecoder {}

#[allow(dead_code)]
impl TorrentDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parser(&self, file_path: PathBuf) -> Torrent {
        let content = fs::read(file_path).unwrap();
        serde_bencode::from_bytes(&content).unwrap()
    }

    pub fn build_tracker_url(&self, torrent: TorrentFile) -> String {
        let url = torrent.announce;
        // Temporary
        let peer_id = String::from("12312312312");

        format!(
            "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&compact={}&left={}",
            url,
            String::from_utf8_lossy(&torrent.info_hash),
            peer_id,
            8080,
            0,
            0,
            1,
            0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_decode_torrent_file() {
        let file_path = PathBuf::new().join("debian-12.6.0-amd64-DVD-1.iso.torrent");
        let torrent_decoder = TorrentDecoder::new();
        let torrent = torrent_decoder.parser(file_path);

        assert_eq!(
            torrent.info.name,
            "debian-12.6.0-amd64-DVD-1.iso".to_string()
        );
        assert_eq!(torrent.info.piece_length, 262144);
    }

    #[test]
    fn should_build_tracker_url() {
        let file_path = PathBuf::new().join("debian-12.6.0-amd64-DVD-1.iso.torrent");
        let torrent_decoder = TorrentDecoder::new();
        let torrent = torrent_decoder.parser(file_path);
        let torrent_file = torrent.to_torrent_file();
        let tracker_url = torrent_decoder.build_tracker_url(torrent_file);

        assert_eq!(tracker_url, "http://bttracker.debian.org:6969/announce?info_hash=2a15945f14b568bd641d54c07abf0b4a8957386d&peer_id=12312312312&port=8080&uploaded=0&downloaded=0&compact=1&left=0".to_string());
    }
}
