use std::{
    io::{Read, Write},
    net::{TcpStream},
    sync::{mpsc::Sender, Arc},
};

use common::{
    infopacket::InfoPacket, login::Login, nominees::Nominees, packet_traits::ToBytes,
    packet_type::PacketType, payment::Payment, register::Register, vote::Vote,
};

use crate::{ballot_box::BallotBox, data_base::DataBase};

static VOTE_COST: u32 = 100;

pub struct Connection {
    stream: TcpStream,
    username: Option<String>,
    number: u32,
}

impl Connection {
    pub fn new(stream: TcpStream, number: u32) -> Connection {
        Connection {
            stream,
            username: None,
            number,
        }
    }

    pub fn start(
        &mut self,
        data_base: Arc<DataBase>,
        tx: Sender<Vote>,
        ballot_box: &mut Arc<BallotBox>,
    ) -> Result<(),String> {
        info!("Conexión {} inicializada", self.number);
        let mut buffer = [0; 1024];

        while let Ok(size) = self.stream.read(&mut buffer) {
            if size == 0 {
                break;
            }
            let packet_type = PacketType::from_utf8(buffer[0]);
            match packet_type {
                PacketType::Login => {
                    info!("Conexión {} - Se recibio un login", self.number);
                    self.handler_login(Login::from_bytes(buffer.to_vec()), &data_base)?
                }
                PacketType::Register => {
                    info!("Conexión {} - Se recibio un register", self.number);
                    self.handler_register(Register::from_bytes(buffer.to_vec()), &data_base)?
                }
                PacketType::Payment => {
                    info!("Conexión {} - Se recibio un payment", self.number);
                    self.handler_payment(Payment::from_bytes(buffer.to_vec()), &data_base)?
                }
                PacketType::Vote => {
                    info!("Conexión {} - Se recibio un vote", self.number);
                    self.handler_vote(Vote::from_bytes(buffer.to_vec()), &data_base, tx.clone())?
                }
                PacketType::RequestNominees => {
                    info!("Conexión {} - Se recibio un request nominees", self.number);
                    self.handler_request_nominees(ballot_box)?
                }
                PacketType::RequestBalance => {
                    info!("Conexión {} - Se recibio un request balance", self.number);
                    self.handler_request_saldo(&data_base)?
                }
                PacketType::RequestResults => {
                    info!("Conexión {} - Se recibio un request results", self.number);
                    self.handler_request_votes(ballot_box)?
                }
                _ => continue, 
            }
            buffer = [0; 1024];
        
        }
        info!("Conexion {} finalizada", self.number);
        Ok(())

    }

    pub fn handler_login(&mut self, packet: Login, data_base: &Arc<DataBase>) -> Result<(),String> {
        if let Err(e) = data_base.log_new_user(packet.username.clone(), packet.password) {
            return self.write_error(&e)            
        }
        info!(
            "Conexión {} - Se logueo correctamente al cliente: {}",
            self.number, packet.username
        );
        self.username = Some(packet.username);

        self.write_info("Login aceptado")
    }

    pub fn handler_register(&mut self, packet: Register, data_base: &Arc<DataBase>) -> Result<(),String> {
        if let Err(e) =
            data_base.save_new_user(packet.username.clone(), packet.password, packet.email)
        {
            return self.write_error(&e)            
        }
        info!(
            "Conexión {} - Se registro correctamente al cliente: {}",
            self.number, packet.username
        );
        self.username = Some(packet.username);
        self.write_info("Register aceptado")
    }

    pub fn handler_payment(&mut self, packet: Payment, data_base: &Arc<DataBase>) -> Result<(),String> {
        match data_base.update_money(packet.username, packet.amount) {
            Err(e) => {
                return self.write_error(&e)
            }
            Ok(saldo) => {
                info!(
                    "Conexión {} - Se recargo correctamente saldo al cliente: {:?}",
                    self.number, self.username
                );
                let saldo_s = saldo.to_string();
                return self.write_info(&saldo_s)
            }
        }
    }

    pub fn handler_vote(&mut self, packet: Vote, data_base: &Arc<DataBase>, tx: Sender<Vote>) -> Result<(),String> {
        if self.username.is_some() {
            let amount = VOTE_COST * packet.cantidad_votos as u32;
            if let Err(e) = data_base.can_vote(self.username.as_deref().unwrap(), amount) {
                
                info!(
                    "Conexión {} - No fue posible votar para el cliente: {:?}",
                    self.number, self.username
                );
                return self.write_error(&e)
            }
            if tx.send(packet).is_err() {
                return self.write_error("SERVER FATAL ERROR")
            }

            if let Err(e) = data_base.update_user_balance(amount, self.username.as_deref().unwrap())
            {
                println!("No se pudo actualiza el balance");
                return self.write_error(&e);
            }

            self.write_info("VOTE ACCEPTED")
        } else {
            self.write_error("Cliente no logueado")
        }        
        
    }
    pub fn handler_request_saldo(&mut self, data_base: &Arc<DataBase>) -> Result<(),String> {
        if self.username.is_some() {
            let copy_user = self.username.clone().unwrap();
            let money = data_base.get_money(&copy_user).unwrap();            
            info!(
                "Conexión {} - Saldo enviado correctamente al cliente: {:?}",
                self.number, self.username
            );
            self.write_info(&money.to_string())
        } else {
            self.write_error("Cliente no logueado")
        }
        
        
        
    }

    pub fn handler_request_votes(&mut self, ballot_box: &mut Arc<BallotBox>) -> Result<(),String> {
        if self.username.is_some() {
            let votes  = ballot_box.get_votes();        
            info!(
                "Conexión {} - Se registraron votos del cliente: {:?}",
                self.number, self.username
            );
            self.write_info(&votes)
        } else {
            self.write_error("PLEASE LOGIN OR REGISTER")
        }

        
    }

    pub fn handler_request_nominees(&mut self, ballot_box: &mut Arc<BallotBox>) -> Result<(),String> {
        if self.username.is_some() {
            let nominees = ballot_box.get_nominees();
            let nom_pkt = Nominees::new(nominees).to_bytes();
            if self.stream.write(&nom_pkt).is_err() {
                return Err(String::from("Error fatal"));
            }
            info!(
                "Conexión {} - Votos enviados correctamente al cliente: {:?}",
                self.number, self.username
            );
            Ok(())
        } else {
            self.write_error("PLEASE LOGIN OR REGISTER")
        }

        
    }

    pub fn write_error(&mut self, error_msg: &str) -> Result<(),String> {
        let error_pkt = InfoPacket::new(PacketType::Error, error_msg.to_string()).to_bytes();
        if self.stream.write(&error_pkt).is_err(){
            return Err(String::from("Error escribiendo al cliente"));
        }
        Ok(()) 
    }

    pub fn write_info(&mut self, info_msg: &str) -> Result<(),String> {
        let info_pkt = InfoPacket::new(PacketType::Info, info_msg.to_string()).to_bytes();
        if self.stream.write(&info_pkt).is_err(){
            return Err(String::from("Error escribiendo al cliente"));
        }
        Ok(())
    }
}
