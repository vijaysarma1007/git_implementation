use git_implementation::{
    cat_file::cat_file, hash_object::hash_object, init::init, ls_tree::ls_tree,
};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rest_of_args = &args[2..];
    match args[1].as_str() {
        "init" => init(),
        "cat-file" => cat_file(&args[2..]),
        "hash-object" => hash_object(&args[2..]),
        "ls-tree" => ls_tree(rest_of_args),
        _ => println!("unknown command: {}", args[1]),
    }
}
