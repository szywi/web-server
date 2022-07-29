use crate::server::config::ServerConfig;
use crate::threads::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::{fs, process};

pub mod config;

pub struct Server {
    config: ServerConfig,
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            config: ServerConfig::default(),
            listener: None,
        }
    }

    pub fn start(&mut self) {
        if self.listener.is_some() {
            println!("Server is already started!");
            return;
        }

        let listener: TcpListener = TcpListener::bind(self.config.host()).unwrap_or_else(|err| {
            println!("Unable to bind to {}\r\n{}", self.config.host(), err);
            process::exit(1);
        });

        self.listener = Some(listener);
        let pool = ThreadPool::new(self.config.workers);

        for stream in self.listener.as_ref().unwrap().incoming() {
            let stream = stream.unwrap();

            pool.execute(|| {
                handle_connection(stream);
            });
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
