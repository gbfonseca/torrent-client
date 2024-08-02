use std::{fs, path::PathBuf};

use crate::models::torrent::Torrent;

#[allow(dead_code)]
pub struct TorrentDecoder {}

#[allow(dead_code)]
impl TorrentDecoder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parser(&self, file_path: PathBuf) -> Torrent {
        let content = fs::read(file_path).unwrap();

        let torrent: Torrent = serde_bencode::from_bytes(&content).unwrap();

        torrent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_decode_torrent_file() {
        let file_path = PathBuf::new().join("archlinux-2024.07.01-x86_64.iso.torrent");
        let torrent_decoder = TorrentDecoder::new();
        let torrent = torrent_decoder.parser(file_path);

        assert_eq!(
            torrent.info.name,
            "archlinux-2024.07.01-x86_64.iso".to_string()
        );
        assert_eq!(torrent.info.piece_length, 524288);
    }
}
