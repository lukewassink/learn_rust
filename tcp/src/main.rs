use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_stream(stream: TcpStream, id: usize) -> std::io::Result<()> {
    let mut stream = stream;
    let mut in_buf = [0; 128];

    println!("Connection open with client {}", id);

    loop {
        match stream.read(&mut in_buf) {
            Ok(0) => {
                println!("Connection closed with client {}", id);
                break;
            }
            Ok(n) => {
                println!(
                    " from client {} >> {:?}",
                    id,
                    str::from_utf8(in_buf.get(..n).unwrap()).unwrap()
                );
                let out_buf = "\n >> Hello to you too\n\n".as_bytes();
                stream.write_all(out_buf)?;
            }
            Err(e) => {
                println!("Error! {}", e);
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:49152";
    let listener = TcpListener::bind(addr)?;

    println!("Listening on {}...", addr);

    for (id, stream) in listener.incoming().enumerate() {
        thread::spawn(move || handle_stream(stream?, id));
    }

    Ok(())
}
