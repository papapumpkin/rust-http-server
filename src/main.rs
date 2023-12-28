use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream}
};


fn handle_connection(mut stream: TcpStream) -> io::Result<()>{
    let mut buffer = [0; 1024];  // create 1024 0s as buffer

    loop {
        match stream.read(&mut buffer) {
            Ok(_) => {
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                stream.write_all(response.as_bytes())?;
            },
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
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
