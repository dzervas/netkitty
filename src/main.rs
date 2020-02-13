extern crate async_std;
extern crate structopt;

use async_std::prelude::*;
use async_std::net::{SocketAddr, TcpListener, TcpStream};
use async_std::io::{stdin, BufReader};

use futures::{select, FutureExt};
use futures::executor::block_on;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "NetKiTTY")]
struct Args {
    /// Listen mode
    #[structopt(short = "l", long = "listen")]
    listen: bool,

    /// Local port number
    #[structopt(short = "p", long = "port")]
    port: Option<u16>,

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
    remote_port: Option<u16>,
}

async fn async_main(args: Args) {
    let addr = SocketAddr::new(args.remote.parse().unwrap(), if args.listen {
        args.port.expect("Please supply a local port to listen on (-p/--port)")
    } else {
        args.remote_port.expect("Please supply a remote port to connect to (<remote_port>)")
    });

    let mut socket = if args.listen {
        let listener = TcpListener::bind(addr).await
            .expect(format!("Unable to bind to {:?}", addr).as_str());
        let accept = listener.accept().await
            .expect(format!("Unable to accept connection on {:?} bind", addr).as_str());
        accept.0
    } else {
        let connector = TcpStream::connect(addr).await
            .expect(format!("Unable to bind to {:?}", addr).as_str());
        connector
    };

    let socket_clone = socket.clone();
    let mut sock_reader = BufReader::new(&socket_clone).lines();
    let mut stdin_reader = BufReader::new(stdin()).lines();

    loop {
        select! {
            l = sock_reader.next().fuse() => println!("{}", l.unwrap().unwrap()),
            l = stdin_reader.next().fuse() => socket.write_all(format!("{}\n", l.unwrap().unwrap()).as_bytes()).await.unwrap()
        }
    }
}

#[paw::main]
fn main(args: Args) {
    block_on(async_main(args));
}
