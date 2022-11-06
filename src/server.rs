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
            handle_client(client);
        });
        // Err(String::from("Error con el cliente")); // Creo que en error simplemente deberia continuar
    }
}

fn handle_client(client: TcpStream) {
    let mut client_m = client.try_clone().unwrap();
    let mut buffer = [0; 1024];
    // while let Ok(size) = client_m.read(&mut buffer) {
    //     if size == 0 {
    //         break;
    //     }
    //     let first_byte = buffer.0;
    //     // match first_byte {
    //     //     REGISTER =>   , // Handler_registrer()
    //     //     LOGIN => ,
    //     // }
    // }
}
