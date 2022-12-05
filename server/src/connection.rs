use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{mpsc::Sender, Arc},
};

use common::{vote::Vote, packet_type::PacketType, login::Login, register::Register, payment::Payment, infopacket::InfoPacket, packet_traits::ToBytes, nominees::Nominees};

use crate::{ballot_box::BallotBox, data_base::DataBase};



static VOTE_COST: u32 = 100;

// Almacenar datos de la conexión
pub struct Connection {
    stream: TcpStream,
    username: Option<String>,
    number: u32,
}

impl Connection {
    pub fn new(stream: TcpStream,number: u32) -> Connection {
        Connection {
            stream,
            username: None,
            number
        }
    }

    pub fn start(
        &mut self,
        data_base: Arc<DataBase>,
        tx: Sender<Vote>,
        ballot_box: &mut Arc<BallotBox>,
    ) {
        let mut buffer = [0; 1024];

        while let Ok(size) = self.stream.read(&mut buffer) {
            if size == 0 {
                break;
            }
            let aux = buffer[0];
            let first_byte = PacketType::from_utf8(aux);
            match first_byte {
                PacketType::LOGIN => {
                    info!("Conexión {} Se recibio un login",self.number);
                    self.handler_login(Login::from_bytes(buffer.to_vec()), &data_base)
                }
                PacketType::REGISTER => {
                    info!("Conexión {} Se recibio un register",self.number);
                    self.handler_register(Register::from_bytes(buffer.to_vec()), &data_base)
                }
                PacketType::PAYMENT => {
                    info!("Conexión {} Se recibio un payment",self.number);
                    self.handler_payment(Payment::from_bytes(buffer.to_vec()), &data_base)
                }
                PacketType::VOTE => {
                    info!("Conexión {} Se recibio un vote",self.number);
                    self.handler_vote(Vote::from_bytes(buffer.to_vec()), &data_base, tx.clone())
                }
                PacketType::REQUEST => {
                    info!("Conexión {} Se recibio un request",self.number);
                    let mut info_packet = InfoPacket::from_bytes(buffer.to_vec());
                    let msg = info_packet.get_msg();

                    if msg == "Obtener Nominados" {
                        info!("Se solicitaron los nominados");
                        self.handler_request_nominees(ballot_box)
                    } else if msg == "Obtener Votos" {
                        info!("Se solicitaron los votos");
                        self.handler_request_votes(ballot_box)
                    }else if msg == "Obtener Saldo" {
                        info!("Saldo solicitado");
                        self.handler_request_saldo(&data_base)
                    }
                }
                _ => (),
            }
            buffer = [0; 1024];
        }
    }

    pub fn handler_login(&mut self, packet: Login, data_base: &Arc<DataBase>) {
        if let Err(e) = data_base.log_new_user(packet.username.clone(), packet.password) {
            self.write_error(&e);
            return;
        }
        info!("Conexión {} - Se logueo correctamente al cliente: {}",self.number,packet.username);
        self.username = Some(packet.username);
        
        self.write_info("Login aceptado")
    }

    pub fn handler_register(&mut self, packet: Register, data_base: &Arc<DataBase>) {
        if let Err(e) =
            data_base.save_new_user(packet.username.clone(), packet.password, packet.email)
        {
            self.write_error(&e);
            return;
        }
        info!("Conexión {} - Se registro correctamente al cliente: {}",self.number,packet.username);
        self.username = Some(packet.username);
        self.write_info("Register aceptado")
    }

    pub fn handler_payment(&mut self, packet: Payment, data_base: &Arc<DataBase>) {
        match data_base.update_money(packet.username, packet.amount) {
            Err(e) => {
                self.write_error(&e);
            }
            Ok(saldo) => {
                info!("Conexión {} - Se recargo correctamente saldo al cliente: {:?}",self.number,self.username);
                let saldo_s = saldo.to_string();
                self.write_info(&saldo_s)
            }
        }
    }

    pub fn handler_vote(&mut self, packet: Vote, data_base: &Arc<DataBase>, tx: Sender<Vote>) {
        if self.username.is_some() {
            let amount = VOTE_COST * packet.cantidad_votos as u32;
            if let Err(e) = data_base.can_vote(self.username.as_deref().unwrap(), amount) {
                self.write_error(&e);
                println!("No es posible votar");
                return;
            }
            if tx.send(packet).is_err() {
                self.write_error("SERVER FATAL ERROR");
            }

            if let Err(e) = data_base.update_user_balance(amount, self.username.as_deref().unwrap())
            {
                self.write_error(&e);
                println!("No se pudo actualiza el balance");
                return;
            }

            self.write_info("VOTE ACCEPTED")
        }

        // Mandar mensaje si todo ok, err si no esta logueado
    }
    pub fn handler_request_saldo(&mut self, data_base: &Arc<DataBase>) {

        let copy_user = self.username.clone().unwrap();
        let money = data_base.get_money(&copy_user).unwrap();
        let nom_pkt = InfoPacket::new(PacketType::INFO, money.to_string()).to_bytes();
        self.stream.write(&nom_pkt).unwrap();
        info!("Conexión {} - Saldo enviado correctamente al cliente: {:?}",self.number,self.username);
    }

    pub fn handler_request_votes(&mut self, ballot_box: &mut Arc<BallotBox>) {
        if self.username.is_some() {
            let votes = ballot_box.get_votes();
            let nom_pkt = InfoPacket::new(PacketType::INFO, votes).to_bytes();

            self.stream.write(&nom_pkt).unwrap(); // Consultar que hacemos de aca
            println!("Votos enviados correctamente");
            return;
        }

        self.write_error("PLEASE LOGIN OR REGISTER");
    }

    pub fn handler_request_nominees(&mut self, ballot_box: &mut Arc<BallotBox>) {
        if self.username.is_some() {
            let nominees = ballot_box.get_nominees();
            let nom_pkt = Nominees::new(nominees).to_bytes();
            self.stream.write(&nom_pkt).unwrap();
            info!("Conexión {} - Votos enviados correctamente al cliente: {:?}",self.number,self.username);
            return;
        }

        self.write_error("PLEASE LOGIN OR REGISTER");
    }

    pub fn write_error(&mut self, error_msg: &str) {
        let error_pkt = InfoPacket::new(PacketType::ERROR, error_msg.to_string()).to_bytes();
        self.stream.write(&error_pkt).unwrap(); // Consultar que hacemos aca
    }

    pub fn write_info(&mut self, info_msg: &str) {
        let info_pkt = InfoPacket::new(PacketType::INFO, info_msg.to_string()).to_bytes();
        self.stream.write(&info_pkt).unwrap(); // Consultar que hacemos aca
    }
}
