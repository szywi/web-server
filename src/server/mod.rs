use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{fs, process};

pub struct Server {
    host: String,
    listener: Option<TcpListener>,
}

// TODO (P1): Deal with start (create listener), options & shutdown
// TODO (P1): Thread pool
// TODO (P2): Enforce connection limit
impl Server {
    pub fn new() -> Server {
        Server {
            host: "127.0.0.1:7878".to_string(),
            listener: None,
        }
    }

    pub fn start(&mut self) {
        self.listener = Some(TcpListener::bind(&self.host).unwrap_or_else(|err| {
            println!("Unable to bind to {}\r\n{}", self.host, err);
            process::exit(1);
        }));

        if let Some(ref listener) = self.listener {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                handle_connection(stream);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "files/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "files/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
