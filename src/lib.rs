mod types;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener};
use std::thread;
pub use types::*;

const TCP_BUFFER_SIZE: usize = 1024; // needs to accommodate the biggest request!

pub fn run_tcp_server<
    'de: 'a,
    'a,
    Payload: DeserializeOwned + Debug,
    SuccessResult: Serialize + Debug,
>(
    tcp_listener: &'a TcpListener,
    req_handler: impl RequestHandler<Payload, SuccessResult> + Send + Sync + Copy + 'static,
) {
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    // handle_client(stream)
                    let mut data = [0 as u8; TCP_BUFFER_SIZE];
                    while match stream.read(&mut data) {
                        Ok(size) => {
                            // echo everything!
                            println!("getting request {}", size);
                            let request =
                                serde_json::from_slice::<Request<Payload>>(&data[0..size])
                                    .expect("Failed to serde_json::from_slice()");
                            println!("got request");

                            let calc_result = req_handler.handle(request.payload);

                            let response = Response {
                                id: request.id,
                                result: match calc_result {
                                    Ok(res) => Result::Success(res),
                                    Err(err) => Result::Err(err),
                                },
                            };

                            let response_str = serde_json::to_string(&response)
                                .expect("Failed to serde_json::to_string()");
                            stream
                                .write(response_str.as_bytes())
                                .expect("Failed to stream.write()");
                            false
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
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
}
