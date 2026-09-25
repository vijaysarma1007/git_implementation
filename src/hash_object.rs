use std::io::Write;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use hex::ToHex;
use sha1::{self, Digest};

pub fn hash_object(args: &[String]) {
    match args[0].as_str() {
        "-w" => {
            let file_name = &args[1];
            println!("Attempting to open path: {:?}", file_name);
            let mut file = std::fs::read(file_name).unwrap();
            let sha = get_sha(&file);
            create_folder(&sha);
            compress(&mut file);
            print_sha(&sha);
        }
        _ => eprintln!("unknown option"),
    }
}

fn get_sha(file: &[u8]) -> String {
    let mut hasher = sha1::Sha1::new();
    hasher.update(file);
    hasher.finalize().encode_hex::<String>()
}

fn compress(file: &mut [u8]) {
    let mut encoded = ZlibEncoder::new(file, Compression::default());
}

fn create_folder(sha: &str) {}

fn print_sha(sha: &str) {
    println!("{sha}");
}

//34:15