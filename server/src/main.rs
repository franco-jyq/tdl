use std::io;

use server::Server;
pub mod server;
pub mod threadpool;
pub mod connection;
pub mod ballot_box;
pub mod data_base;
pub mod user;

fn main() -> io::Result<()> {
    let mut server = Server::new();
    server.start_server()
}
