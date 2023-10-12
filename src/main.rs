use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs
};
use hello_server::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4); // setting thread pool size as 4 initially
    for stream in listener.incoming() {
        thread_pool.execute(|| {
            handle_connection(stream.unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let request= BufReader::new(&mut stream)
        .lines().next().unwrap().unwrap();

    let (status, path) = if request == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let html_contents = fs::read_to_string(path).unwrap();
    let content_len = html_contents.len();
    // write response header status\r\nContent-Length: len\r\n\r\nContent
    let response =
        format!("{status}\r\nContent-Length: {content_len}\r\n\r\n{html_contents}");
    // write response
    stream.write_all(response.as_bytes()).unwrap();
}
