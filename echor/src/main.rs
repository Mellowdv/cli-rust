use clap::{Command, Arg, ArgAction};

fn print_type_of<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}

fn main() {
    let matches = Command::new("echor")
        .version("0.1")
        .about("Rust version of `echo`")
        .arg(Arg::new("Text")
             .value_name("TEXT")
             .help("Input text")
             .required(true)
             .num_args(1..))
        .arg(Arg::new("No newline")
             .short('n')
             .help("Do not print a new line")
             .action(ArgAction::SetTrue))
        .get_matches();

    let input_text: Vec<&str> = matches.get_many::<String>("Text")
        .unwrap()
        .map(|s| s.as_str())
        .collect();

    let ending = if matches.get_flag("No newline") { "" } else { "\n" };
    print!("{}{}", input_text.join(" "), ending);
}
