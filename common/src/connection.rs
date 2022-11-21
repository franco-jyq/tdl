use std::{io::Read, net::TcpStream, sync::Arc};

use crate::{data_base::DataBase, packet_type::PacketType, payment::Payment, register::Register};

// Almacenar datos de la conexión
pub struct Connection {
    stream: TcpStream,
    //logged: bool,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            //logged: false,
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
                    println!("Se recibio un register");
                    self.handler_register(Register::from_bytes(buffer.to_vec()), &data_base)
                }
                PacketType::PAYMENT => {
                    println!("Se recibio un pago");
                    self.handler_payment(Payment::from_bytes(buffer.to_vec()), &data_base)
                }
                _ => (),
            }
            buffer = [0; 1024];
        }
    }

    pub fn handler_register(&self, packet: Register, data_base: &Arc<DataBase>) {
        data_base
            .save_new_user(packet.username, packet.password, packet.email)
            .unwrap();
        println!("Se registro correctamente al cliente");
        // Aca yo diria de ver si es error(mandar error) o mandar ok
    }

    pub fn handler_payment(&self, packet: Payment, data_base: &Arc<DataBase>) {
        data_base
            .update_money(packet.username, packet.amount)
            .unwrap();
        println!("Se recargo correctamente saldo")
        // Lo mismo aca
    }
}
