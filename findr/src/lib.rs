use std::error::Error;
use clap::{Command, Arg, ArgAction};
use regex::Regex;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    patterns: Vec<Regex>,
    types: Vec<String>,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("findr")
        .author("t.talik")
        .version("0.1.0")
        .about("Subset of functionality of find, written in rust, for fun and profit")
        .arg(
            Arg::new("PATH")
                .action(ArgAction::Append)
                .default_value(".")
                .num_args(0..)
        )
        .arg(
            Arg::new("NAME")
                .short('n')
                .long("name")
                .action(ArgAction::Append)
                .value_parser(Regex::new)
                .num_args(0..)
        )
        .arg(
            Arg::new("TYPE")
                .short('t')
                .long("type")
                .action(ArgAction::Append)
                .value_parser(["d", "f", "l"])
                .num_args(0..)
        )
        .get_matches();
    Ok(Config {
        paths: matches
                .get_many::<String>("PATH")
                .unwrap()
                .map(|s| s.to_string())
                .collect(),
        patterns: matches.get_many::<Regex>("NAME")
                .unwrap_or_default()
                .map(|p| Regex::new(p.as_str()).unwrap())
                .collect(),
        types: match matches.get_many::<String>("TYPE") {
            Some(m) => m.map(|s| s.to_string()).collect(),
            _ => vec![],
        },
    })
}

fn visit_dirs(cfg: &Config, node: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {

    let mut matched: bool = false;
    let final_part = node.file_name().unwrap();
    if cfg.patterns.iter().any(|p| p.is_match(final_part.to_str().expect("NOT FOUND!"))) || cfg.patterns.len() == 0 {
        matched = true;
    }

    if node.is_dir() {
        if matched &&
            (cfg.types.iter().any(|s| s == "d") ||
             cfg.types.len() == 0) {
            println!("{}", node.display()); 
        }
        for entry in std::fs::read_dir(node)? {
            let entry = entry?;
            let path = entry.path();
            if let Err(e) = visit_dirs(cfg, &path) {
                eprintln!("{}: {}", path.display(), e);
            }
        }
    }

    if node.is_file() {
        if matched &&
            (cfg.types.iter().any(|s| s == "f") ||
             cfg.types.len() == 0) {
            println!("{}", node.display()); 
        }
        return Ok(())
    }

    if node.is_symlink() {
        if matched &&
            (cfg.types.iter().any(|s| s == "l") ||
             cfg.types.len() == 0) {
            println!("{}", node.display()); 
        }
        return Ok(())
    }
    Ok(())
}
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    for p in &cfg.paths {
        let path = Path::new(p);
        if let Err(e) = std::fs::metadata(path) {
            eprintln!("{}: {}", path.display(), e);
            continue;
        }
        if let Err(e) = visit_dirs(&cfg, path) {
            eprintln!("{}: {}", path.display(), e);
        }
    }
    Ok(())
}
