use std::{io::Read, net::TcpStream, sync::Arc};

use crate::{register::Register,packet_type::PacketType, data_base::DataBase};

// Almacenar datos de la conexión
pub struct Connection {
    stream: TcpStream,
    logged: bool,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            logged: false,
        }
    }

    pub fn start(&mut self, data_base: Arc<DataBase>) {
        let mut buffer = [0; 1024];
        while let Ok(size) = self.stream.read(&mut buffer) {
            
            if size == 0 {                
                break;
            }
            let aux = buffer[0];
            let first_byte = PacketType::from_utf8(aux);
            println!("The first byte is {:?}", first_byte);
            match first_byte {
                PacketType::REGISTER => {
                    println!("tuki");
                    self.handler_register(Register::from_bytes(buffer.to_vec()),&data_base)
                }
                _ => (),
            }
            buffer = [0;1024];
            
        }
    }

    pub fn handler_register(&self, packet: Register, data_base: &Arc<DataBase>) {
        
        data_base.save_new_user(packet.username, packet.password, packet.email).unwrap();
        println!("Se registro correctamente al cliente");
        
    }
}
