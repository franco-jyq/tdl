pub(crate) use gh22::connection::Connection;
use gh22::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::str;

static PORT: &str = "127.0.0.1:8095";
static THREAD_NUMBER: usize = 10;

pub struct Server {
    listener: TcpListener
}

impl Server {

    pub fn new() -> Self {
        Server {
            listener = None
        }
    }

    pub fn star_server(&mut self) {
        if let Ok(listener) = TcpListener::bind(PORT) {
            self.listener = listener;
            obtain_connections(self)
        }
        Err(String::from("Error inicializando servidor"))
    }

    fn obtain_connections(&mut self) {
        let pool = ThreadPool::new(THREAD_NUMBER);
        for client in self.listener.incoming().flatten() {
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
}


