use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::net::{TcpListener, TcpStream};
use tcp_server::{send_tcp_msg, start_tcp_server, RequestHandler};

#[derive(Debug, Serialize, Deserialize)]
pub struct SumRequest {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SumResult {
    pub res: u8,
}

#[derive(Copy, Clone)]
pub struct SumRequestHandler {}

impl RequestHandler<SumRequest, SumResult> for SumRequestHandler {
    fn handle(&self, req: SumRequest) -> Result<SumResult, String> {
        req.x
            .checked_add(req.y)
            .map(|x| SumResult { res: x })
            .ok_or_else(|| format!("Failed to sum {} and {}", req.x, req.y))
    }
}

// https://blog.logrocket.com/command-line-argument-parsing-rust-using-clap/
#[derive(Parser, Debug)]
#[clap(author = "Borboletinha", version, about)]
struct Arguments {
    #[clap(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Client,
    Server,
}

fn main() {
    let args = Arguments::parse();
    match args.cmd {
        SubCommand::Server => {
            println!("server...");
            let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
            start_tcp_server(&listener, SumRequestHandler {});
        }
        SubCommand::Client => {
            println!("client...");
            match TcpStream::connect("localhost:3333") {
                Ok(stream) => send_tcp_msg::<SumRequest, SumResult>(&stream, SumRequest { x: 25, y: 45 }),
                Err(e) => println!("Failed to connect: {}", e),
            }
            println!("Terminated.");
        }
    }
}
