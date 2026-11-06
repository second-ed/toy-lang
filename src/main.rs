use toy_lang::interpret;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();
    let source = std::fs::read_to_string(args.path).expect("Expected .toy file");
    match interpret(&source) {
        Ok(_) => {}
        Err(e) => eprintln!(e),
    }
}
