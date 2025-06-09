use clap::Parser;

/// Command line arguments parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Path of the file to be read as a string
    pub file_path: String,
}
