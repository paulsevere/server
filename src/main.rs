use std::net::{TcpListener, TcpStream};
use std::iter::Iterator;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    // let conns = listener.incoming().map(|stream| );
}


fn handle_client(stream: TcpStream) {
    println!("{:?}", stream);
}

// // fn dump()