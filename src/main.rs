use crate::server::Server;

mod server;
mod threads;

fn main() {
    let mut server: Server = Server::new();
    server.start();
}
