use std::net::{TcpListener, TcpStream};
use std::io::{self, Read};


fn handle_connection(mut stream: TcpStream) -> io::Result<()>{
    let mut buffer = [0; 1024];  // create 1024 0s in buffer

    loop {
        let bytes_read = stream.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        println!("HTTP/1.1 200 OK\r\n\r\n")
    }
    Ok(())
}

fn main() -> io::Result<()> {
    println!("Starting server...");

    let listener = TcpListener::bind("127.0.0.1:4221")?;

    println!("Server listening on port 4221...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_connection(stream)?;
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
    Ok(())
}
