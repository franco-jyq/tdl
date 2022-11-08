use std::{io::Read, net::TcpStream};

use crate::{register::Register,packet_type::PacketType};

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

    pub fn start(&mut self) {
        let mut buffer = [0; 1024];
        while let Ok(size) = self.stream.read(&mut buffer) {
            if size == 0 {
                break;
            }
            let aux = buffer[0];
            let first_byte = PacketType::from_utf8(aux);
            match first_byte {
                PacketType::REGISTER => {
                    self.handler_register(Register::from_bytes(buffer.to_vec()))
                }
                _ => (),
            }
        }
    }

    pub fn handler_register(&self, packet: Register) {}
}
