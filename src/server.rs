use gh22::connection::Connection;
use gh22::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::str;

static PORT: &str = "127.0.0.1:8095";
static THREAD_NUMBER: usize = 10;

fn main() {
    start_server().unwrap();
}

fn start_server() -> Result<(), String> {
    // Creo que estaria bien dejar elegir al puerto para configurar el server
    if let Ok(listener) = TcpListener::bind(PORT) {
        obtain_connections(listener)
    }
    Err(String::from("Error inicializando servidor"))
}

fn obtain_connections(listener: TcpListener) {
    let pool = ThreadPool::new(THREAD_NUMBER);
    for client in listener.incoming().flatten() {
        println!("Recibi conexión");
        pool.execute(|| {
            spawn_connection(client);
        });
        // Err(String::from("Error con el cliente")); // Creo que en error simplemente deberia continuar
    }
}

fn spawn_connection(client: TcpStream) {
    let client_m = client.try_clone().unwrap();
    let mut connection = Connection::new(client_m);
    connection.start();
}
