use clap::{Arg, ArgAction, Command};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};

pub struct Config {
    input_file: String,
    output_file: Option<String>,
    count: bool,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("uniqr")
        .author("t.talik")
        .version("0.1.0")
        .about("simple clone of uniq, only -c available")
        .arg(
            Arg::new("IN_FILE")
                .default_value("-")
                .action(ArgAction::Set)
                .help("Input file [default: -]"),
        )
        .arg(
            Arg::new("OUT_FILE")
                .default_value(None)
                .action(ArgAction::Set)
                .help("Output file"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Show counts"),
        )
        .get_matches();

    Ok(Config {
        input_file: matches.get_one::<String>("IN_FILE").unwrap().to_string(),
        output_file: matches.get_one::<String>("OUT_FILE").cloned(),
        count: matches.get_flag("count"),
    })
}

fn get_count_string(flag: bool, count: i32) -> String {
    if !flag {
        return "".to_string();
    }

    return format!("{:>4} ", count);
}

fn output(out_file: &str, line: &str, count_flag: bool, count: i32) -> Result<(), Box<dyn Error>>{
    if out_file != "-" {
        let mut handle = File::options().append(true).open(out_file)?;
        let new_line = format!("{}{}", get_count_string(count_flag, count), line);
        handle.write(new_line.as_bytes());
    } else {
        print!("{}{}", get_count_string(count_flag, count), line);
    }
    Ok(())
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut reader: Box<dyn BufRead> =
        if cfg.input_file == "-" {
            Box::new(std::io::BufReader::new(std::io::stdin()))
        } else {
            let handle = File::open(&cfg.input_file)
                .map_err(|e| format!("{}: {}", cfg.input_file, e))?;
            Box::new(BufReader::new(handle))
        };

    let mut line: String = Default::default();
    let mut previous: String = "".to_string();
    let mut count = 0;
    let output_file: String = match cfg.output_file {
        Some(f) => f,
        None => "-".to_string()
    };
    loop {
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            if previous == "" { break; }
            output(&output_file, &previous, cfg.count, count);
            break;
        }

        if line.trim() != previous.trim() {
            if previous != "" {
                output(&output_file, &previous, cfg.count, count);
                count = 0;
            }
            previous = line.clone();
        }
        count += 1;
        line.clear();
    }
    Ok(())
}
