#![allow(unused)]
use anyhow::{Context, Result};
use clap::Parser;
use env_logger;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, warn};
use std::error;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};

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

fn do_hard_work() -> Result<String, String> {
    let delay_duration = Duration::from_millis(50);
    sleep(delay_duration);

    // Simulate fetching data (replace with your actual data retrieval code)
    let data = "Simulated data fetched after 5 seconds".to_string();

    // Return the fetched data or an error
    Ok(data)
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    let args = Cli::parse();
    let path = &args.path.into_os_string().into_string().unwrap();
    let content =
        fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;

    // Showing a progress bar
    let pb = ProgressBar::new(10);

    for i in 0..10 {
        do_hard_work();
        pb.inc(1);
        if i == 5 {
            warn!("almost there...");
        }
    }
    pb.finish_with_message("done");

    println!("file content: {}", content);

    // A note on printing performance
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = io::BufWriter::new(&stdout); // optional: wrap that handle in a buffer
    writeln!(handle, "handle: {}", 42)?; // add `?` if you care about errors here

    let mut handle = stdout.lock(); // acquire a lock on it
    writeln!(handle, "handle lock: {}", 84)?; // add `?` if you care about errors here
    info!("finished");

    Ok(())
}
