use ut3::server;
use ut3::client;

use std::io;

use clap::Parser;

#[derive(Parser)]
struct Flags {
    #[clap(long = "server")]
    server: bool,

    #[clap(short = 'p', long = "port", default_value = "3333")]
    port: String,

    #[clap(long="ip", default_value="localhost")]
    ip: String,
}

fn main() -> io::Result<()> {
    let args = Flags::parse();

    if args.server {
        server::run(args.port)
    } else {
        client::run(args.ip, args.port)
    }
}
