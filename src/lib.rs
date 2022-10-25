mod types;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
pub use types::*;

const TCP_BUFFER_SIZE: usize = 1024;

pub fn start_tcp_server<'a, Payload: DeserializeOwned + Debug, SuccessResult: Serialize + Debug>(
    tcp_listener: &'a TcpListener,
    req_handler: impl RequestHandler<Payload, SuccessResult> + Send + Sync + Copy + 'static,
) {
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().expect("Failed to connect"));
                thread::spawn(move || {
                    let mut data = [0 as u8; TCP_BUFFER_SIZE];
                    let mut all_data = Vec::<u8>::new();

                    while match stream.read(&mut data) {
                        // triggered if data buffer is filled
                        Ok(TCP_BUFFER_SIZE) => {
                            all_data.extend_from_slice(&data);
                            true
                        }
                        // triggered once OEF is received
                        Ok(size) => {
                            all_data.extend_from_slice(&data[0..size]);
                            let response = match serde_json::from_slice::<Request<Payload>>(&all_data) {
                                Ok(request) => Response {
                                    id: request.id,
                                    result: Result::Success(req_handler.handle(request.payload)),
                                },
                                Err(err) => Response {
                                    id: 0,
                                    result: Result::Err(format!("Error on json unmarshall: {}", err)),
                                },
                            };
                            let response_str = serde_json::to_string(&response).expect("Failed to serde_json::to_string()");
                            stream.write(response_str.as_bytes()).expect("Failed to stream.write()");
                            false
                        }
                        Err(_) => {
                            println!("Terminating connection due to {}", stream.peer_addr().expect("Failed to obtain peer address"));
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
