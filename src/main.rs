// use std::net::{TcpListener, TcpStream};
extern crate mio;
use std::net::*;
use mio::*;
use mio::tcp::{TcpListener, TcpStream};
use std::io::Write;
use mio::tcp::Shutdown::Both;



// use std::iter::Iterator;

fn main() {
    const SERVER: Token = Token(0);
    // const CLIENT: Token = Token(1);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let server = TcpListener::bind(&addr).unwrap();

    let poll = Poll::new().unwrap();

    // Start listening for incoming connections
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();
    let mut events = Events::with_capacity(1024);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    println!("Handling connection!");
                    let (mut socket, addr) = server.accept().unwrap();
                    let writeResult = socket.write(httpResponse().as_bytes());
                    match writeResult {
                        Ok(_) => {
                            println!("Shutting down");

                            socket.shutdown(Both);

                        }
                        Err(_) => {}
                    }

                }
                _ => unreachable!(),
            }
        }
    }


}


pub fn httpResponse() -> String {

    let message = format!(r#"HTTP-Version = HTTP/2.0
Content-Type: text/html

{}
    "#,
                          "WOW");
    return String::from(message);
}