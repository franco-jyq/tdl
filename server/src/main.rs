use std::io;
use server::Server;
pub mod server;
pub mod threadpool;
pub mod connection;
pub mod ballot_box;
pub mod data_base;
pub mod user;

#[macro_use]
extern crate log;

fn main() -> io::Result<()> {
    env_logger::init();
    //let mut builder = Builder::from_default_env();
    //builder.
    info!("Inicializando Servidor");
    let mut server = Server::new();
    server.start_server()
}
