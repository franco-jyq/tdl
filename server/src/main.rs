use server::Server;
pub mod ballot_box;
pub mod connection;
pub mod data_base;
pub mod server;
pub mod threadpool;
pub mod user;

#[macro_use]
extern crate log;

fn main() {
    env_logger::builder().format_target(false).init();
    info!("Inicializando Servidor");
    let mut server = Server::new().unwrap();
    server.start_server()
}
