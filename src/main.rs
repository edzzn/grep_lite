use clap::{Arg, Command};
use regex::Regex;

fn main() {
    let matches = Command::new("gep-lite")
        .version("1.0")
        .arg(
            Arg::new("pattern")
                .short('p')
                .long("pattern")
                .help("Patter to search"),
        )
        .get_matches();

    let pattern = matches
        .get_one::<String>("pattern")
        .expect("Missing pattern");

    let re = Regex::new(pattern).unwrap();
    let text = "\
    Every face, every shop,
    bedroom window, public-house, and
    dark square is a picture
    feverishly turned--in search of what?
    It is the same with books.
    What do we seek
    through millions of pages?";

    for (i, line) in text.lines().enumerate() {
        match re.find(line) {
            Some(_) => println!("{}: {}", i, line),
            _ => (),
        }
    }
}
