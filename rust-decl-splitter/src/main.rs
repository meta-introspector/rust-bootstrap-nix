use std::env;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_directory>", args[0]);
        return;
    }
    let input_dir = PathBuf::from(&args[1]);

    for entry in WalkDir::new(&input_dir) {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }
}
