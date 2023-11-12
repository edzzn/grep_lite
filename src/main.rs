use clap::{Arg, Command};
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("gep-lite")
        .version("1.0")
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .help("Patter to search")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .help("File to search")
                .required(true),
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .expect("Missing pattern");
    let re = Regex::new(pattern)?;

    let file = matches
        .get_one::<String>("input")
        .expect("Missing input file");
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let line_num = i + 1;
        if let Some(_) = re.find(&line) {
            println!("{}: {}", line_num, line);
        }
    }

    Ok(())
}
