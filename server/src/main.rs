use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // accept connections and process them serially
    for result in listener.incoming() {
        match result {
            Ok(stream) => handle_client(stream)?,
            Err(e) => connection_error(e)?,
        }
    }

    Ok(())
}

fn connection_error(e: Error) -> std::io::Result<()> {
    println!("couldn't get client: {e:?}");
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::new(1, 0)))?;

    let mut buffer = [0; 10];

    // read up to 10 bytes
    let n = stream.read(&mut buffer[..])?;
    println!("n: {n:?} buffer {buffer:?}");

    stream.write_fmt(format_args!(
        "HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nze\r\n"
    ))?;

    println!("Msg sent");

    sleep(Duration::new(1, 0));

    Ok(())
}
