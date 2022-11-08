use std::io;
use std::{io::Write, net::TcpStream};

pub struct Client {
        stream: TcpStream
}

impl Client {
    
    pub fn new(stream: TcpStream) -> Self {

        Client {
            stream
        }
    }

    pub fn escribir_mensaje(&mut self) {
        let mut command: String = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        command.remove(command.len() - 1);
        command.push('\r');
        command.push('\n');
        let mensaje = command.clone();
        match self.stream.write(mensaje.as_bytes()) {
            Err(_) => println!("Fallo conexion con servidor"),
            Ok(_) => {
                if self.stream.flush().is_err() {
                    println!("Error con flush")
                }
            }
        }
    }
}


