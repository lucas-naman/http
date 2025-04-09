use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    stream.write_fmt(format_args!("2"))?;

    println!("message sent");

    println!("listening");

    let mut buffer = [0; 10];

    loop {
        let n = stream.read(&mut buffer[..])?;

        println!("n: {n:?} buffer {buffer:?}");

        if n < 10 {
            break;
        }
    }
    // read up to 10 bytes

    Ok(())
}
