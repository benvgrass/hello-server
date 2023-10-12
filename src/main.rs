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

    let response_status = "HTTP/1.1 200 OK";
    let html_contents = fs::read_to_string("hello.html").unwrap();
    let content_len = html_contents.len();
    // write response header status\r\nContent-Length: len\r\n\r\nContent
    let response =
        format!("{response_status}\r\nContent-Length: {content_len}\r\n\r\n{html_contents}");
    // write response
    stream.write_all(response.as_bytes()).unwrap();
}
