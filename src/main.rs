use toy_lang::interpret;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

use std::path::PathBuf;

fn cargo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn main() {
    // let args = Args::parse();
    // let source = std::fs::read_to_string(args.path).expect("Expected .toy file");
    let source = std::fs::read_to_string(cargo_root().join("toy/if_else_branch.toy"))
        .expect("Expected .toy file");
    match interpret(&source) {
        Ok(()) => {}
        Err(e) => eprintln!("{e}"),
    }
}
