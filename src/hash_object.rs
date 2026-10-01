use flate2::Compression;
use flate2::write::ZlibEncoder;
use hex::ToHex;
use sha1::{self, Digest};
use std::{
    fs::DirBuilder,
    io::Write,
    path::{Path, PathBuf},
};

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let file_name = &args[1];
            let file = std::fs::read(file_name).unwrap();
            let header = get_header(&file);
            let mut content = header.into_bytes();
            content.extend(file);
            let sha = get_sha(&content);
            print_sha(&sha);
            let folder = create_folder(&sha);
            let compressed_file = compress(&content);
            let file_sha = get_file_sha(&sha);
            save_file(&compressed_file, folder, &file_sha);
        }
        _ => eprintln!("unknown option"),
    }
}

fn get_sha(file: &[u8]) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(file);
    hasher.finalize().encode_hex::<String>()
}

fn compress(file: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(file).unwrap();
    encoder.finish().unwrap()
}

fn create_folder(sha: &str) -> PathBuf {
    let path = Path::new(".git").join("objects").join(&sha[0..2]);
    DirBuilder::new().recursive(true).create(&path).unwrap();
    path
}

fn print_sha(sha: &str) {
    println!("{sha}");
}

fn get_header(content: &[u8]) -> String {
    let object_type = "blob";
    let size = content.len();
    format!("{} {}\0 ", object_type, size)
}

fn save_file(file: &[u8], mut path: PathBuf, file_sha: &str) {
    path.push(file_sha);
    let path = Path::new(&path);
    if path.exists() {
        return;
    }
    std::fs::write(path, file).unwrap();
}

fn get_file_sha(sha: &str) -> &str {
    &sha[2..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_provide_file_sha() {
        let sha = "2qedhfgkjdshfglkjerhgejfge65";
        let expected_file_sha = "edhfgkjdshfglkjerhgejfge65";
        let result = get_file_sha(sha);
        assert_eq!(result, expected_file_sha);
    }

    #[test]
    fn should_create_blob_header() {
        let content = "what is up, doc?";
        let expected_result = "blob 16\0";
        let result = get_header(content.as_bytes());
        assert_eq!(result, expected_result);
    }
}
