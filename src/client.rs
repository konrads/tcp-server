use crate::types;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

pub fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = types::Address {
                street: "Bennett St".to_owned(),
                city: "Sydney".to_owned(),
            };
            let msg_str = serde_json::to_string(&msg).unwrap();
            let msg_b = msg_str.as_bytes();

            stream.write(msg_b).unwrap();
            println!("Sent {}, awaiting reply...", msg_str);

            let mut data = [0_u8; 1000];
            match stream.read(&mut data) {
                Ok(size) => {
                    // let filtered_b: Vec<u8> = data.into_iter().take_while(|x| *x != 0).collect();
                    println!(
                        "...got: {}: {}",
                        data.len(),
                        from_utf8(&data[0..size]).unwrap()
                    );
                    let address = serde_json::from_slice::<types::Address>(&data[0..size]).unwrap();
                    println!("Got address: {:?}", address);
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
