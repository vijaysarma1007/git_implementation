use flate2::bufread::ZlibDecoder;
use std::{env, fs, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "cat-file" {
        if args[2] == "-p" {
            let hash = args[3].as_str();
            let folder_name = &hash[0..2];
            let file_name = &hash[2..];
            let path = format!(".git/objects/{}/{}", folder_name, file_name);
            let mut object = fs::File::open(path).unwrap();
            let mut content: Vec<u8> = vec![];
            let mut extracted_content = String::new();
            object.read_to_end(&mut content).unwrap();
            let mut decoder = ZlibDecoder::new(content.as_slice());
            decoder.read_to_string(&mut extracted_content).unwrap();

            println!("{extracted_content}");
        }
    }
}
