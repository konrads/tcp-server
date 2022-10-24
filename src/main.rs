use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use tcp_server::{run_tcp_server, Request, RequestHandler, Response};

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
            run_tcp_server(&listener, SumRequestHandler {});
        }
        SubCommand::Client => {
            println!("client...");
            match TcpStream::connect("localhost:3333") {
                Ok(mut stream) => {
                    println!("Successfully connected to server in port 3333");

                    let req = Request {
                        id: 55,
                        payload: SumRequest { x: 25, y: 45 },
                    };
                    let req_str = serde_json::to_string(&req).unwrap();
                    let req_b = req_str.as_bytes();

                    stream.write(req_b).unwrap();
                    println!("Sent {}, awaiting reply...", req_str);

                    let mut data = [0_u8; 1000];
                    match stream.read(&mut data) {
                        Ok(size) => {
                            println!("...got text: {}: {}", data.len(), from_utf8(&data[0..size]).unwrap());
                            let response = serde_json::from_slice::<Response<SumResult>>(&data[0..size]).unwrap();
                            println!("Got response: {:?}", response);
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            }
            println!("Terminated.");
        }
    }
}
