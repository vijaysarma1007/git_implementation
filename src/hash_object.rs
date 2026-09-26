use flate2::Compression;
use flate2::write::ZlibEncoder;
use hex::ToHex;
use sha1::{self, Digest};
use std::{
    fs::DirBuilder,
    io::{Write},
};

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let file_name = &args[1];
            println!("Attempting to open path: {:?}", file_name);
            let file = std::fs::read(file_name).unwrap();
            let sha = get_sha(&file);
            let folder = create_folder(&sha);
            let compressed_file = compress(&file);
            print_sha(&sha);
            let file_sha = get_file_sha(&sha);
            save_file(&compressed_file, &folder, &file_sha);
            
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

fn create_folder(sha: &str) -> String {
    let path = format!(".git/objects/{}", &sha[..2]);
    DirBuilder::new().recursive(true).create(&path).unwrap();
    path
}

fn print_sha(sha: &str) {
    println!("{sha}");
}

fn save_file(file: &[u8], folder_path: &str, file_sha: &str) {
    let path = format!("{}/{}", folder_path, file_sha);
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
}
