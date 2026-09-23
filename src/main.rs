use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "cat-file" {
        if args[2] == "-p"{
            let hash = args[3].as_str();
            let folder_name = &hash[0..2];
            let file_name = &hash[2..];
            dbg!(hash);
    }
}
}