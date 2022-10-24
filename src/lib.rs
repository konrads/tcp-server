mod types;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
pub use types::*;

const TCP_BUFFER_SIZE: usize = 1024; // FIXME: currently accommodating for the entire message...

pub fn start_tcp_server<'a, Payload: DeserializeOwned + Debug, SuccessResult: Serialize + Debug>(
    tcp_listener: &'a TcpListener,
    req_handler: impl RequestHandler<Payload, SuccessResult> + Send + Sync + Copy + 'static,
) {
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    let mut data = [0 as u8; TCP_BUFFER_SIZE];
                    while match stream.read(&mut data) {
                        Ok(size) => {
                            let request = serde_json::from_slice::<Request<Payload>>(&data[0..size]).expect("Failed to serde_json::from_slice()");
                            let calc_result = req_handler.handle(request.payload);

                            let response = Response {
                                id: request.id,
                                result: match calc_result {
                                    Ok(res) => Result::Success(res),
                                    Err(err) => Result::Err(err),
                                },
                            };

                            let response_str = serde_json::to_string(&response).expect("Failed to serde_json::to_string()");
                            stream.write(response_str.as_bytes()).expect("Failed to stream.write()");
                            false
                        }
                        Err(_) => {
                            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
                            stream.shutdown(Shutdown::Both).unwrap();
                            false
                        }
                    } {}
                });
            }
            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    }
}

// FIXME: need to rework in terms of a struct with methods:
// - new(&stream: TcpStream, response_callback: Fn<R>(resp: R) => ()
// - send<P: Serialize + Debug>(&self, msg) => ()
pub fn send_tcp_msg<P: Serialize + Debug, R: DeserializeOwned + Debug>(mut stream: &TcpStream, payload: P) {
    let req = Request { id: 55, payload: payload };

    let req_str = serde_json::to_string(&req).unwrap();
    let req_b = req_str.as_bytes();

    stream.write(req_b).unwrap();
    println!("Sent {}, awaiting reply...", req_str);

    let mut data = [0_u8; 1000];
    match stream.read(&mut data) {
        Ok(size) => {
            // println!("...got text: {}: {}", data.len(), from_utf8(&data[0..size]).unwrap());
            let response = serde_json::from_slice::<Response<R>>(&data[0..size]).unwrap();
            println!("Got response: {:?}", response);
        }
        Err(e) => {
            println!("Failed to receive data: {}", e);
        }
    }
}
