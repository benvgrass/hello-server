use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request: Vec<_> = BufReader::new(&mut stream)
        .lines().map(|r| r.unwrap())
        .take_while(|l| !l.is_empty()).collect();

    println!("Request: {:#?}", request);

}
