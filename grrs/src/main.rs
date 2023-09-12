#![allow(unused)]
use anyhow::{Context, Result};
use clap::Parser;
use grrs::{do_hard_work, find_matches};
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use termimad::crossterm::style::{Attribute::*, Color::*};
use termimad::*;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,
}

impl Cli {
    fn new(pattern: String, path: PathBuf) -> Self {
        Cli { pattern, path }
    }

    fn test() -> String {
        String::from("test method")
    }
}

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting up...");

    // read file
    let args = Cli::parse();
    let path = &args.path.into_os_string().into_string().unwrap();
    let content =
        fs::read_to_string(path).with_context(|| format!("Could not read file `{}`", path))?;

    // show a progress bar
    let pb = ProgressBar::new(10);
    for i in 0..10 {
        do_hard_work();
        pb.inc(1);
        if i == 5 {
            warn!("⚠️  Wait.. almost there...");
        }
    }
    pb.finish_with_message("done");

    // termimad - print pretty markdown on terminal
    let mut skin = MadSkin::default(); // start with the default skin
    skin.bold.set_fg(gray(20)); // let's decide bold is in light gray
    skin.strikeout = CompoundStyle::new(Some(Red), None, Bold.into());
    eprintln!("\n\n{}", skin.term_text(&content));

    // show pattern matched
    info!("Data matched with '{}' \n", &args.pattern);
    find_matches(&content, &args.pattern, &mut std::io::stdout());

    info!("🤯 Creating some markdown file and writing data on it");
    let mut file = fs::File::create("./random2.md")?; // Create or open a file for writing
    let bytes_written = file.write(content.as_bytes())?; // Write the data to the file
    println!("Wrote {} bytes", bytes_written);

    info!("✅ Finished");

    Ok(())
}
