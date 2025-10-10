use clap::{Command, Arg, ArgAction};
use std::error::Error;
use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::string::String;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("Good ol' me")
        .about("wc, except rust")
        .arg(Arg::new("files")
             .default_value("-")
             .action(ArgAction::Append)
             .help("Input file(s)"))
        .arg(Arg::new("lines")
             .short('l')
             .long("lines")
             .help("Show line count")
             .action(ArgAction::SetTrue))
        .arg(Arg::new("words")
             .short('w')
             .long("words")
             .help("Show word count")
             .action(ArgAction::SetTrue))
        .arg(Arg::new("bytes")
             .short('c')
             .long("bytes")
             .help("Show byte count")
             .action(ArgAction::SetTrue)
             .conflicts_with("chars"))
        .arg(Arg::new("chars")
             .short('m')
             .long("chars")
             .help("Show character count")
             .action(ArgAction::SetTrue))
        .get_matches();

    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    let flags = vec![lines, words, bytes, chars];
    if !flags.into_iter().any(|flag| flag) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_string())
            .collect(), 
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(file_path: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match file_path {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_path)?))),
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    let num_of_files = config.files.len();
    let mut line = String::new();
    for file in config.files {
        let mut handle;
        match open(&file) {
            Ok(h) => handle = h,
            Err(e) => {
                eprintln!("{}: {}", file, e);
                continue;
            }
        }

        let mut lines = 0;
        let mut words = 0;
        let mut bytes = 0;
        let mut chars = 0;
        loop {
            let read_bytes = handle.read_line(&mut line)?;
            if read_bytes == 0 {
                break;
            }

            lines += 1;
            words += line.split_whitespace().count();
            bytes += read_bytes;
            chars += line.chars().count();
            line.clear();
        }
        total_lines += lines;
        total_words += words;
        total_bytes += bytes;
        total_chars += chars;
        if config.lines { print!("{:8}", lines); }
        if config.words { print!("{:8}", words); }
        if config.bytes { print!("{:8}", bytes); }
        if config.chars { print!("{:8}", chars); }
        if file != "-" { 
            println!(" {}", file);
        } else {
            println!("");
        }
    }
    if num_of_files > 1 {
        if config.lines { print!("{:8}", total_lines); }
        if config.words { print!("{:8}", total_words); }
        if config.bytes { print!("{:8}", total_bytes); }
        if config.chars { print!("{:8}", total_chars); }
        println!(" total");
    }
    Ok(())
}
