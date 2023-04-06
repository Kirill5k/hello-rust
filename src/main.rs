use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello_rust::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    listener
        .incoming()
        .into_iter()
        .for_each(|stream| pool.execute(|| handle_connection(stream.unwrap())))
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = http_request.first().unwrap();

    let response = match &request_line[..] {
        "GET / HTTP/1.1" => default_success_response(),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            default_success_response()
        }
        _ => not_found_response(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn default_success_response() -> String {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let content_length = format!("Content-Length: {}", contents.len());
    format!("{status_line}\r\n{content_length}\r\n\r\n{contents}")
}

fn not_found_response() -> String {
    let status_line = "HTTP/1.1 404 Not Found";
    let contents = fs::read_to_string("notfound.html").unwrap();
    let content_length = format!("Content-Length: {}", contents.len());
    format!("{status_line}\r\n{content_length}\r\n\r\n{contents}")
}
