use std::io::{Error, Read};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // accept connections and process them serially
    for result in listener.incoming() {
        match result {
            Ok(stream) => handle_client(stream),
            Err(e) => connection_error(e),
        }
    }
}

fn connection_error(e: Error) {
    println!("couldn't get client: {e:?}")
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();

    let n = stream.read_to_string(&mut buffer).expect("coulndt read");

    println!("n: {n}/read: {buffer:?}")
}
