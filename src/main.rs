#![allow(unused)]
use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    pattern : String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {

        if line.contains(&(args.pattern)) {
            println!("{}", line);
        }
    }

}
