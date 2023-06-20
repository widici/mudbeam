use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The address of the host to trace can be a hostname or a ipv4 address
    #[arg(index = 1)]
    pub addr: String,

    /// The max ttl value
    #[arg(long, short, default_value_t = 64)]
    pub max_ttl: u8,

    /// The start ttl value
    #[arg(long, short, default_value_t = 1)]
    pub start_ttl: u8,

    /// The max rtt value in secs before a timeout occurs
    #[arg(long, short, default_value_t = 3)]
    pub timeout: u64,

    /// The amount of times a ttl gets pinged
    #[arg(long, short, default_value_t = 3)]
    pub n_attempts: u64,
}

pub fn parse() -> Args {
    return Args::parse()
}