// src/main.rs
/*
 * Main executable for GoTestnetToolkitPlatformUltra
 */

use clap::Parser;
use gotestnettoolkitplatformultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "GoTestnetToolkitPlatformUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
