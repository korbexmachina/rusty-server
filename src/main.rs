use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rusty_server::ThreadPool;

fn main() {
    // bind to port
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // listen for connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // handle connections
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // read http request
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // assign content
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // format response
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

    // send response
    stream.write_all(response.as_bytes()).unwrap();
}
