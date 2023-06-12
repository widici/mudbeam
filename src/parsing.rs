use clap::{arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    let args = Command::new("mudbeam")
        .arg(arg!([IP]))
        .get_matches();

    return args
}