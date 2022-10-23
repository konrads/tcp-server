use crate::types;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use tcp_server::SumResult;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 1000]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            // stream.write(b"cheers for message:").unwrap();
            let address = serde_json::from_slice::<types::Address>(&data[0..size]).unwrap();
            let address2 = types::Address {
                city: "Gold Coast".to_owned(),
                ..address
            };
            let msg_str = serde_json::to_string(&address2).unwrap();
            let msg_b = msg_str.as_bytes();

            stream.write(msg_b).unwrap();
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
