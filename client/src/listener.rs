use std::{ net::TcpStream, sync::mpsc::{Sender,Receiver}, time::Duration};

const TIMEOUT_NANO:u32 = 10000000;

pub struct Listener {
    tx: Sender<String>,
    rx: Receiver<String>,
    stream:TcpStream
}

impl Listener {
    
    pub fn new(tx:Sender<String>,rx:Receiver<String>,stream:TcpStream) -> Self{
        Listener {
            tx,
            rx,
            stream
        }
    }

    pub fn escuchar_server(&mut self){
        loop{
            //let mut buf = [0;PACKET_SIZE];
    
            //let _leido_server = stream.read(&mut buf).unwrap();
    
            if  let Ok(leido_cliente) = self.rx.recv_timeout(Duration::new(0, TIMEOUT_NANO)){
                println!("{leido_cliente}");
                break; 
            }
        }
    }
    
}