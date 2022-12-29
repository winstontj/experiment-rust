use std::{path, fs, io};
use anyhow::{Ok, Context};
use clap::Parser;

// Search pattern in a particular file and display that line. Similar to how grep works
#[derive(Parser)]
struct Cli {
    // The pattern to look
    pattern: String,
    // The path to the file
    path: path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    println!("Arg: pattern: {0} path: {1}", args.pattern, args.path.to_str().unwrap());
    let file_content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {}", &args.path.to_str().unwrap()))?;
    grepclone::find_matches(&file_content, &args.pattern, &mut io::stdout());

    Ok({})
}
