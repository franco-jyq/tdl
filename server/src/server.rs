use common::ballot_box::{self, BallotBox};
use common::connection::Connection;
use common::data_base::DataBase;
use common::vote::Vote;

use crate::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::{str, io, thread};

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
            let data_base_arc = Arc::new(data_base);
            
            if let Ok(ballot_box) = BallotBox::load_ballot("./src/ballot_data_base".to_string()){
                let mut ballot_box_arc = Arc::new(ballot_box);

                let (tx, rx) = mpsc::channel();
                launch_main_handler(&mut ballot_box_arc, rx).unwrap();
   
                match TcpListener::bind(PORT) {
                    Ok(listener) => self.listener = Some(listener),    
                    Err(e) => return Err(e),
                };
                self.obtain_connections(data_base_arc, tx)?; // Pensar resultado

            }
        };

        Ok(())
    }

    fn obtain_connections(&mut self, data_base: Arc<DataBase>, tx: Sender<Vote>) -> io::Result<()>{

        
        let pool = ThreadPool::new(THREAD_NUMBER);
        for client in self.listener.as_ref().unwrap().incoming().flatten() {
            let tx_clone = tx.clone();
            println!("Recibi conexión");
            let db_clone = data_base.clone();
            pool.execute( move | | {
                spawn_connection(client, db_clone ,tx_clone);
            });
            //Err(String::from("Error con el cliente")); // Creo que en error simplemente deberia continuar
        }
        Ok(())
    }
    
}

fn spawn_connection(client: TcpStream, data_base: Arc<DataBase>, tx: Sender<Vote>) {
    
    if let Ok(client_m) = client.try_clone(){
        let mut connection = Connection::new(client_m);
        connection.start(data_base, tx);
    }
}


fn launch_main_handler(ballot_box: &mut Arc<BallotBox>,rx: Receiver<Vote>) -> Result<(), String> {
    let ballot_box_reference = ballot_box.clone();
    let _join_handler: thread::JoinHandle<_> = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(vote) => {
                    ballot_box_reference.vote_nominee(vote.nominado).unwrap();
                    if let Ok(nominees) = ballot_box_reference.nominees.read() {
                    println!("{:?}", nominees);
                    };
                    
                }, 
                Err(_) => println!("Error "),
            }
        }
    });
    Ok(())
}


             // if let Ok(nominees) = ballot_box.nominees.read() {
                //     println!("{:?}", nominees);
                // }
                // ballot_box.vote_nominee("Franco".to_string()).unwrap();
                // if let Ok(nominees) = ballot_box.nominees.read() {
                //     println!("{:?}", nominees);
                // }
