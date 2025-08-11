use std::fs;

use clap::{Parser, Subcommand};
use docs_legend::generate_site;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "docslegend", version = VERSION, about = "Generate documentation from input '.dl' files")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Build { file: String, outdir: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Build { file, outdir }) => {
            match generate_site(
                &file.to_owned(),
                &outdir,
                &fs::read_to_string(file).expect("Err"),
            ) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
        _ => {
            println!("unkown");
        }
    }
}
