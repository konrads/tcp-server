use clap::{Parser, Subcommand};
mod types;

#[derive(Parser, Debug)]
#[clap(author = "Borboletinha", version, about)]
/// A Very simple Package Hunter
struct Arguments {
    #[clap(subcommand)]
    cmd: SubCommand,
}

// https://blog.logrocket.com/command-line-argument-parsing-rust-using-clap/

#[derive(Subcommand, Debug)]
enum SubCommand {
    Client,
    Server,
}

mod client;
mod server;

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Client => {
            println!("client...");
            client::main()
        }
        SubCommand::Server => {
            println!("server!!!");
            server::main()
        }
    }
}
