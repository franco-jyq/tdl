use std::{ net::TcpStream, sync::mpsc::{Sender,Receiver}, time::Duration, io::Read};

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
            let mut buffer = [0; 1024];
    
            if let Ok(size) = self.stream.read(&mut buffer){
                println!("{size}");
                //cuando escucha algo del servidor se lo manda al cliente
                if let Ok(_read) = self.tx.send(String::from_utf8(buffer.to_vec()).unwrap()){
                    continue;
                }else{
                    break;
                }
            }
            if  let Ok(leido_cliente) = self.rx.recv_timeout(Duration::new(0, TIMEOUT_NANO)){
                println!("{leido_cliente}");
                break; 
            }
        }
    }
    
}