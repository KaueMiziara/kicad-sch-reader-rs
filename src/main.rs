mod utils;

use clap::Parser;
use utils::Args;

fn main() {
    let file_path = match Args::try_parse() {
        Ok(args) => args.file_path,
        Err(_) => {
            println!("File path not informed");
            return;
        }
    };
    println!("Path: {}", file_path);

    if !file_path.ends_with("kicad_sch") {
        println!("The selected file is not a Kicad Schema");
        return;
    }

    let content = match std::fs::read_to_string(&file_path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read file {file_path}"),
    };

    utils::parse_sexp(&content);
}
