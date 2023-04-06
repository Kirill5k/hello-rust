use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    listener
        .incoming()
        .into_iter()
        .for_each(|stream| handle_connection(stream.unwrap()))
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let content_length = format!("Content-Length: {}", contents.len());

    let response = format!("{status_line}\r\n{content_length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
