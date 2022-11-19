
use std::{io::Write, net::TcpStream};

use common::register::Register;

pub struct Client {
        stream: TcpStream
}

impl Client {
    
    pub fn new(address:String) -> Result<Self, ()> {
        
        if let Ok(stream) = TcpStream::connect(address) {
            println!("Connectado al servidor!");
            return Ok(Client {
                stream : stream
            })
        }else {
            println!("No se pudo conectar...");
            return Err(())
        }
    }

    pub fn escribir_mensaje(&mut self,vec_msg:Vec<&str>) {
        //let mut command: String = String::new();
        
        if let Ok(mut register_pak) = Register::new("franco".to_string(),"123".to_string(),"5decopas@gmail.com".to_string()){

            match self.stream.write(register_pak.to_bytes().as_slice()) {
                Err(_) => println!("Fallo conexion con servidor"),
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        println!("Error con flush")
                    }
                }
            }
        }
    }
}


