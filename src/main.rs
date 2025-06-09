use clap::Parser;

/// Clap test program
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    name: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello, {}!", args.name);
}
