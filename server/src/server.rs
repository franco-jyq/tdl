use common::connection::Connection;

use crate::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::{str, io};

static PORT: &str = "127.0.0.1:8095";
static THREAD_NUMBER: usize = 10;

pub struct Server {
    listener: Option<TcpListener>
}

impl Server {

    pub fn new() -> Self {
        Server {
            listener: None
        }
    }

    pub fn start_server(&mut self) -> io::Result<()>{
        
       match TcpListener::bind(PORT) {
            Ok(listener) => self.listener = Some(listener),    
            Err(e) => return Err(e),
        };

        self.obtain_connections()

    }

    fn obtain_connections(&mut self) -> io::Result<()>{

        let pool = ThreadPool::new(THREAD_NUMBER);
        
        for client in self.listener.as_ref().unwrap().incoming().flatten() {
            println!("Recibi conexión");
            pool.execute(|| {
                spawn_connection(client);
            });
            //Err(String::from("Error con el cliente")); // Creo que en error simplemente deberia continuar
        }
        Ok(())
    }
    
}

fn spawn_connection(client: TcpStream) {
    let client_m = client.try_clone().unwrap();
    let mut connection = Connection::new(client_m);
    connection.start();
}
