use std::error::Error;
use std::fmt;
use clap::{Command, Arg, ArgAction, value_parser};
use std::num::ParseIntError;

#[derive(Clone)]
pub struct CutRange {
    start: usize,
    end: usize,
}

// Not sure how much of this is actually needed
#[derive(Debug)]
pub struct CutRangeError;
impl fmt::Display for CutRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing a range!")
    }
}
impl Error for CutRangeError {}
impl From<ParseIntError> for CutRangeError {
    fn from(_: ParseIntError) -> Self {
        return CutRangeError {};
    }
}

pub fn parse_cutranges(string: &str) -> Result<Vec<CutRange>, CutRangeError> {
    let mut parsed: Vec<CutRange> = vec![];
    for rng in string.split(',') {
        let start_and_end: Vec<_> = rng.split('-').collect();
        if start_and_end.len() == 1 {
            let start_and_end = start_and_end[0].parse::<usize>()?;
            parsed.push(CutRange {
                start: start_and_end,
                end: start_and_end,
            })
        } else {
            parsed.push(CutRange {
                start: start_and_end[0].parse::<usize>()?,
                end: start_and_end[1].parse::<usize>()?, });
        }
    }
    Ok(parsed)
}

enum CutType {
   Byte,
   Character,
   Field,
   Invalid,
}

pub struct Config {
    files: Vec<String>,
    cut_type: CutType,
    delimiter: char,
    list: Vec<CutRange>,
}

pub fn get_args() -> Result<Config, Box<dyn Error>> {
    let matches = Command::new("cutr")
        .version("0.1.0")
        .author("t.talik")
        .about("Clone of cut in Rust, for fun and profit")
        .arg(
            Arg::new("files")
                .default_value("-")
        )
        .arg(
            Arg::new("bytes")
                .short('b')
                .long("bytes")
                .conflicts_with_all(["characters", "fields"])
                .value_parser(parse_cutranges)
        )
        .arg(
            Arg::new("characters")
                .short('c')
                .long("characters")
                .conflicts_with_all(["bytes", "fields"])
                .value_parser(parse_cutranges)
        )
        .arg(
            Arg::new("fields")
                .short('f')
                .long("fields")
                .conflicts_with_all(["characters", "bytes"])
                .value_parser(parse_cutranges)
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("delimiter")
                .value_parser(value_parser!(char))
                .default_value("\t")
        )
        .get_matches();

    let mut cut_type: CutType = CutType::Invalid;
    let mut ranges: Vec<CutRange> = vec![];
    if let Some(v) = matches.get_one::<Vec<CutRange>>("bytes") {
        ranges = v.clone();
        cut_type = CutType::Byte;
    } else if let Some(v) = matches.get_one::<Vec<CutRange>>("characters") {
        ranges = v.clone();
        cut_type = CutType::Character;
    } else if let Some(v) = matches.get_one::<Vec<CutRange>>("fields") {
        ranges = v.clone();
        cut_type = CutType::Field;
    }

    Ok(Config {
        files: matches
            .get_many("files")
            .unwrap()
            .cloned()
            .collect(),
        cut_type: cut_type,
        delimiter: *matches
            .get_one("delimiter")
            .unwrap(),
        list: ranges,
    })
}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    Ok(())
}
