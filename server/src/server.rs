use common::connection::Connection;
use common::data_base::DataBase;

use crate::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
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
    // . ./src/data_file
    pub fn start_server(&mut self) -> io::Result<()>{
        
        if let Ok(data_base) = DataBase::new("./src/data_file"){
            println!("sdasd");
            let data_base_arc = Arc::new(data_base);
            match TcpListener::bind(PORT) {
                Ok(listener) => self.listener = Some(listener),    
                Err(e) => return Err(e),
            };

            self.obtain_connections(data_base_arc)?; // Pensar resultado
        };

        Ok(())
    }

    fn obtain_connections(&mut self, data_base: Arc<DataBase>) -> io::Result<()>{

        
        let pool = ThreadPool::new(THREAD_NUMBER);
        
        for client in self.listener.as_ref().unwrap().incoming().flatten() {
            println!("Recibi conexión");
            let db_clone = data_base.clone();
            pool.execute( move | | {
                spawn_connection(client, db_clone );
            });
            //Err(String::from("Error con el cliente")); // Creo que en error simplemente deberia continuar
        }
        Ok(())
    }
    
}

fn spawn_connection(client: TcpStream, data_base: Arc<DataBase>) {
    
    if let Ok(client_m) = client.try_clone(){
        let mut connection = Connection::new(client_m);
        connection.start(data_base);
    }
    
}
