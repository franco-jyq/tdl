use std::{io::{Read, Write}, net::TcpStream, sync::{Arc, mpsc::Sender}};

use crate::{data_base::DataBase, packet_type::PacketType, payment::Payment, register::Register, vote::Vote, infopacket::InfoPacket};

static VOTE_COST: u32 = 100;

// Almacenar datos de la conexión
pub struct Connection {
    stream: TcpStream,
    username: Option<String>
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            username: None
        }
    }

    pub fn start(&mut self, data_base: Arc<DataBase>, tx: Sender<Vote>) {
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
                },
                PacketType::VOTE => {
                    println!("Se recibio un voto");
                    self.handler_vote(Vote::from_bytes(buffer.to_vec()), &data_base, tx.clone())
                },
                PacketType::INFO => {
                    println!("Se solicitaron los nominados");
                    self.handler_request(&data_base,tx.clone())
                }
                _ => (),
            }
            buffer = [0; 1024];
        }
    }

    pub fn handler_register(&mut self, packet: Register, data_base: &Arc<DataBase>) {
        if let Err(e) =  data_base.save_new_user(packet.username.clone(), packet.password, packet.email) {
            self.write_error(&e);
            return;
        }
        self.username = Some(packet.username);    
        println!("Se registro correctamente al cliente");        
    }

    pub fn handler_payment(&mut self, packet: Payment, data_base: &Arc<DataBase>) {
        if let Err(e) =   data_base.update_money(packet.username, packet.amount) {
            self.write_error(&e);
            return;
        }
            
        println!("Se recargo correctamente saldo")
    }

    pub fn handler_vote(&mut self, packet: Vote, data_base: &Arc<DataBase>, tx: Sender<Vote>) {
        if self.username.is_some(){
            if let Err(e) = data_base.can_vote(self.username.as_deref().unwrap(), VOTE_COST * packet.cantidad_votos as u32) {
                self.write_error(&e);
                return; 
            }
            if tx.send(packet).is_err(){
                self.write_error("SERVER FATAL ERROR");
            }
        }        
        
        
        
    }

    pub fn handler_request(&mut self,data_base: &Arc<DataBase>, tx: Sender<Vote>) {

    }

    pub fn write_error(&mut self,error_msg: &str)  {
        let error_pkt = InfoPacket::new(PacketType::ERROR,error_msg.to_string()).to_bytes();
        self.stream.write(&error_pkt).unwrap(); // Consultar que hacemos aca  
    }
}
