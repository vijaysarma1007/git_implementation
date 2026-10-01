use std::io::Read;

use flate2::bufread::ZlibDecoder;



pub fn get_object_directory_name(hash: &str) -> String {
    hash[0..2].to_owned()
}

pub fn get_object_file_name(hash: &str) -> String {
    hash[2..].to_owned()
}

pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(bytes);
    let mut result = vec![];
    decoder.read_to_end(&mut result).unwrap();
    result
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::{get_object_directory_name, get_object_file_name};
    use std::io::Write;
use flate2::{Compression, write::{ZlibEncoder}};

    #[test]
    fn should_get_object_directory_name_from_hash() {
        let hash = "85jwhflwehjrflwekfjwelk98y3344qa";
        let expected_directory_name = "85";
        let name = get_object_directory_name(hash);
        assert_eq!(name, expected_directory_name);
    }

    #[test]
    fn should_get_the_file_name() {
        let hash = "85jwhflwehjrflwekfjwelk98y3344qa";
        let expected_hash = "jwhflwehjrflwekfjwelk98y3344qa";
        let name = get_object_file_name(hash);
        assert_eq!(name, expected_hash);
    }

    #[test]
    fn should_decompress() {
        let de_comppressed_string = "85jwhflwehjrflwekfjwelk98y3344qa";
        //compress
        let mut encoder = ZlibEncoder::new(vec![], Compression::default());
        encoder.write_all(de_comppressed_string.as_bytes()).unwrap();
        let compressed = encoder.finish().unwrap();
        //de compress
        let decompressed = decompress(&compressed);
        assert_eq!(decompressed, de_comppressed_string.as_bytes());
    }
}
