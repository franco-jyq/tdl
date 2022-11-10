use std::io;

use server::Server;
pub mod server;
pub mod threadpool;

fn main() -> io::Result<()>{
    let mut server = Server::new();
    server.start_server()
}


