use clap::{Command, Arg, ArgAction};
use std::error::Error;
use std::fs::read_to_string;
use std::io;
use std::io::Read;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool
}

type MyResult<T> = Result<T, Box<dyn Error>>;
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .about("Just a clone of cat")
        .arg(Arg::new("file")
             .default_value("-")
             .action(ArgAction::Append))
        .arg(Arg::new("number lines")
             .short('n')
             .long("number")
             .action(ArgAction::SetTrue))
        .arg(Arg::new("number non-blank lines")
             .short('b')
             .long("number-nonblank")
             .action(ArgAction::SetTrue))
        .get_matches();

    Ok(Config {
        files: matches
                .get_many::<String>("file")
                .unwrap()
                .map(|s| s.to_string())
                .collect(),
        number_lines: matches.get_flag("number lines"),
        number_nonblank: matches.get_flag("number non-blank lines"),
    })
}

fn print_file(f: &str, cfg: &Config) -> MyResult<()> {
    let mut buffer = String::new();
    if f == "-" {
        let mut buf = vec![];
        match io::stdin().read_to_end(&mut buf) {
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
            Ok(_) => buffer = String::from_utf8(buf)?
        }
    } else {
        match read_to_string(f) {
            Err(e) => eprintln!("Failed to open {}: {}", f, e),
            Ok(file_contents) => buffer = file_contents
        }
    }

    let mut line_number = 1;
    for line in buffer.lines() {
        if line == "" && cfg.number_nonblank {
            println!();
            continue;
        }
        if cfg.number_lines || cfg.number_nonblank {
            print!("     {}\t", line_number);
            line_number += 1;
        }
        println!("{}", line);
    }
    Ok(())
}

pub fn run(cfg: Config) -> MyResult<()> {
    for file in &cfg.files {
        let _ = print_file(file, &cfg);
    }
    Ok(())
}
