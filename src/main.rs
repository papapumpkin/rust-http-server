use std::net::TcpListener;

fn handle_connection(stream: TcpStream) {
    match stream.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }
 }

fn main() -> std::io::Result<()> {
    println!("Starting server...");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    println!("Server listening on port 4221...");

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
