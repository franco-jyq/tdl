use common::vote::Vote;

use crate::ballot_box::BallotBox;
use crate::connection::Connection;
use crate::data_base::DataBase;
use crate::threadpool::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::{io, str, thread};

static PORT: &str = "127.0.0.1:8095";
static THREAD_NUMBER: usize = 10;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new() -> io::Result<Server> {
        Ok(Server {
            listener: TcpListener::bind(PORT).unwrap(),
        })
    }

    pub fn start_server(&mut self) {
        let data_base = DataBase::new("./src/data_file").unwrap();
        let data_base_arc = Arc::new(data_base);

        let ballot_box = BallotBox::load_ballot("./src/ballot_data_base".to_string()).unwrap();
        let mut ballot_box_arc = Arc::new(ballot_box);

        let (tx, rx) = mpsc::channel();
        launch_main_handler(&mut ballot_box_arc.clone(), rx).unwrap();

        self.obtain_connections(data_base_arc, tx, &mut ballot_box_arc);
        // Pensar resultado
    }

    fn obtain_connections(
        &mut self,
        data_base: Arc<DataBase>,
        tx: Sender<Vote>,
        ballot_box: &mut Arc<BallotBox>,
    ) {
        info!("Escuchando conexiones");
        let pool = ThreadPool::new(THREAD_NUMBER);
        for (nro_connection, client) in self.listener.incoming().flatten().enumerate() {
            let tx_clone = tx.clone();
            let mut ballot_clone = ballot_box.clone();
            info!("Conexión recibida");
            let db_clone = data_base.clone();
            pool.execute(move || {
                spawn_connection(
                    client,
                    db_clone,
                    tx_clone,
                    &mut ballot_clone,
                    nro_connection.try_into().unwrap(),
                );
            });
            //Err(String::from("Error con el cliente")); // Creo que en error simplemente deberia continuar
        }
    }
}

fn spawn_connection(
    client: TcpStream,
    data_base: Arc<DataBase>,
    tx: Sender<Vote>,
    ballot_box: &mut Arc<BallotBox>,
    nro_connection: u32,
) {
    if let Ok(client_m) = client.try_clone() {
        let mut connection = Connection::new(client_m, nro_connection);
        connection.start(data_base, tx, ballot_box);
    }
}

fn launch_main_handler(ballot_box: &mut Arc<BallotBox>, rx: Receiver<Vote>) -> Result<(), String> {
    let ballot_box_reference = ballot_box.clone();
    let _join_handler: thread::JoinHandle<_> = thread::spawn(move || loop {
        match rx.recv() {
            Ok(vote) => {
                ballot_box_reference
                    .vote_nominee(vote.nominado, vote.cantidad_votos.into())
                    .unwrap();
                if let Ok(nominees) = ballot_box_reference.nominees.read() {
                    println!("{:?}", nominees);
                };
            }
            Err(_) => println!("Error "),
        }
    });
    Ok(())
}
