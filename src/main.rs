#![allow(unused)]
use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser)]
struct Cli {
    pattern : String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let args = Cli::parse();
    let content = read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}",&args.path.display(), err)))?;


    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())

}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {

    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

#[test]
fn find_a_match() {

    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}