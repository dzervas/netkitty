extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "netkitty")]
struct Opts {
    /// Listen mode
    #[structopt(short = "l", long = "listen")]
    listen: bool,

    /// Local port number
    #[structopt(short = "p", long = "port")]
    port: Option<u32>,

    /// Randomize local and remote ports
    #[structopt(long = "randomize")]
    randomize: bool,

    /// Put local TTY in Raw mode
    #[structopt(short = "r", long = "raw")]
    raw: bool,

    /// Local source address (ip or hostname)
    #[structopt(short = "s", long = "source")]
    source: Option<String>,

    /// Set verbosity level (pass it up to 3 times)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: i32,

    /// Print version information and exit
    #[structopt(short = "V", long = "version")]
    version: bool,

    /// Remote target (ip or hostname)
    remote: String,

    /// Remote port to connect to
    remote_port: Option<u32>,
}

fn main() {
    let opts: Opts = Opts::from_args();

    println!("{:?}", opts);
}
