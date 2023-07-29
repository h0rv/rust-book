use multi_threaded_server::ThreadPool;

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        let status_line = "HTTP/1.1 200 ok";
        let filename = "static/index.html";
        (status_line, filename)
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        let status_line = "HTTP/1.1 200 ok";
        let filename = "static/index.html";
        (status_line, filename)
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let filename = "static/404.html";
        (status_line, filename)
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents,
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
