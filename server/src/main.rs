use server::Server;
pub mod Server;

fn main() {
    let server = Server::new();
    server.start_server()
}


