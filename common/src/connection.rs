use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::{mpsc::Sender, Arc},
};

use crate::{
    ballot_box::BallotBox, data_base::DataBase, infopacket::InfoPacket, nominees::Nominees,
    packet_type::PacketType, payment::Payment, register::Register, vote::Vote, login::Login,
};

static VOTE_COST: u32 = 100;

// Almacenar datos de la conexión
pub struct Connection {
    stream: TcpStream,
    username: Option<String>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            username: None,
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
            println!("The first byte is {:?}", first_byte);
            match first_byte {
                PacketType::LOGIN => {
                    println!("Se recibio un login");
                    self.handler_login(Login::from_bytes(buffer.to_vec()),&data_base)
                }
                PacketType::REGISTER => {
                    println!("Se recibio un register");
                    self.handler_register(Register::from_bytes(buffer.to_vec()), &data_base)
                }
                PacketType::PAYMENT => {
                    println!("Se recibio un pago");
                    self.handler_payment(Payment::from_bytes(buffer.to_vec()), &data_base)
                }
                PacketType::VOTE => {
                    println!("Se recibio un voto");
                    self.handler_vote(Vote::from_bytes(buffer.to_vec()), &data_base, tx.clone())
                }
                PacketType::REQUEST => {
                    println!("Se solicitaron los nominados");
                    self.handler_request(ballot_box)
                }
                _ => (),
            }
            buffer = [0; 1024];
        }
    }

    pub fn handler_login(&mut self,packet: Login,data_base: &Arc<DataBase>){
        if let Err(e) =
            data_base.log_new_user(packet.username.clone(), packet.password)
        {
            self.write_error(&e);
            return;
        }
        self.username = Some(packet.username);
        println!("Se logueo correctamente al cliente");
        self.write_info("LOGIN ACCEPTED")
    }

    pub fn handler_register(&mut self, packet: Register, data_base: &Arc<DataBase>) {
        if let Err(e) =
            data_base.save_new_user(packet.username.clone(), packet.password, packet.email)
        {
            self.write_error(&e);
            return;
        }
        self.username = Some(packet.username);
        println!("Se registro correctamente al cliente");
        self.write_info("REGISTRATION ACCEPTED")
    }

    pub fn handler_payment(&mut self, packet: Payment, data_base: &Arc<DataBase>) {
        match  data_base.update_money(packet.username, packet.amount){
            Err(e) => {
                self.write_error(&e);
                return;
            },
            Ok(saldo) => {
                println!("Se recargo correctamente saldo");
                let saldo_s = saldo.to_string();
                self.write_info(&saldo_s)
            }       
        }

        
    }

    pub fn handler_vote(&mut self, packet: Vote, data_base: &Arc<DataBase>, tx: Sender<Vote>) {
        if self.username.is_some() {
            if let Err(e) = data_base.can_vote(
                self.username.as_deref().unwrap(),
                VOTE_COST * packet.cantidad_votos as u32,
            ) { 
                self.write_error(&e);
                println!("No es posible votar");
                return;
            }
            if tx.send(packet).is_err() {
                self.write_error("SERVER FATAL ERROR");
            }

            self.write_info("VOTE ACCEPTED")
        }

        // Mandar mensaje si todo ok, err si no esta logueado
    }

    pub fn handler_request(&mut self, ballot_box: &mut Arc<BallotBox>) {
        if self.username.is_some() {
            let nominees = ballot_box.get_nominees();
            let nom_pkt = Nominees::new(nominees).to_bytes();
            self.stream.write(&nom_pkt).unwrap(); // Consultar que hacemos de aca
            println!("Nominados enviados correctamente");
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
