use crate::server::Server;

mod server;

fn main() {
    let mut server: Server = Server::new();
    server.start();
}
