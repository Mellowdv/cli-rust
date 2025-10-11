use clap::{Command, Arg, ArgAction};
use std::error::Error;
use std::fs::read_to_string;
use std::io;
use std::io::Read;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: i32,
    bytes: Option<i32>
}

fn parse_positive_int(to_parse: &str) -> Result<i32, Box<dyn Error>> {
    match to_parse.parse() {
        Ok(x) if x > 0 => Ok(x),
        _ => Err(From::from(to_parse))
    }
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .about("Just a clone of head")
        .arg(Arg::new("file")
            .default_value("-")
            .action(ArgAction::Append))
        .arg(Arg::new("LINES")
            .short('n')
            .long("lines")
            .default_value("10")
            .action(ArgAction::Set))
        .arg(Arg::new("BYTES")
            .short('c')
            .long("bytes")
            .action(ArgAction::Set)
            .conflicts_with("LINES"))
        .get_matches();

    let lines = matches
        .get_one::<String>("LINES")
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .map_err(|e| format!("error: invalid value '{}' for '--lines <LINES>': invalid digit found in string", e))?;

    let bytes = matches
        .get_one::<String>("BYTES")
        .map(|s| parse_positive_int(s.as_str()))
        .transpose()
        .map_err(|e| format!("error: invalid value '{}' for '--bytes <BYTES>': invalid digit found in string", e))?;

    Ok(Config {
        files: matches
            .get_many::<String>("file")
            .unwrap()
            .map(|s| s.to_string())
            .collect(),
        lines: lines.unwrap(),
        bytes
    })
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let num_of_files = cfg.files.len();
    let multiple_files: bool = if num_of_files > 1 { true } else { false };

    for (index, file) in cfg.files.into_iter().enumerate() {
        if multiple_files {
            println!("==> {} <==", file);
        }

        let mut buffer = String::new();
        if file == "-" {
            let mut buf = vec![];
            match io::stdin().read_to_end(&mut buf) {
                Err(e) => eprintln!("{}", e),
                Ok(_) => buffer = String::from_utf8(buf)?
            }
        } else {
            match read_to_string(&file) {
                Err(e) => eprintln!("{}: {}", file, e),
                Ok(file_contents) => buffer = file_contents
            }
        }
        match cfg.bytes {
            Some(bytes) => print!("{}", String::from_utf8_lossy(&buffer
                    .as_bytes()
                    .into_iter()
                    .take(bytes as usize)
                    .cloned()
                    .collect::<Vec<u8>>())),
            None => for line in buffer.lines().take(cfg.lines as usize) {
                println!("{}", line);
            }
        }
        if multiple_files && (index < num_of_files - 1) { println!(""); }
    }
    Ok(())
}
