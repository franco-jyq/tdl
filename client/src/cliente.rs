use std::{io::{Write, Read}, net::TcpStream};
use common::{register::Register, packet_type::PacketType, infopacket::InfoPacket};
use common::vote::Vote;

pub struct Client {
        stream: TcpStream
}

impl Client {
    
    pub fn new(stream:TcpStream) -> Self {

        Client {
            stream : stream
        }
    }

    pub fn escribir_mensaje(&mut self,mut vec_msg:Vec<&str>) -> Result<bool,String>{
        
        let value = vec_msg.remove(0);

        match value{
            "Iniciar-Sesion" => (),
            "Registrarse" => return self.registrarse(vec_msg),
            "Votar" => return self.votar(vec_msg),
            "Consultar-Votos" => (),
            "Consultar-Nominados" => return self.consultar_nominados(),
            _ => return {
                println!("Nombre de mensaje inválido, ultilize Ayuda para ver los mensajes disponibles");
                Ok(false)},
        }
        Ok(false)
    }

    fn registrarse(&mut self,mut args:Vec<&str>) -> Result<bool,String>{

        if !(args.len() == 3){
            println!("Para registrarse debe mandar un nombre de usuario,contraseña y mail");
            return Ok(false);
        }

        let username = args.remove(0);
        let password = args.remove(0);
        let email = args.remove(0);

        if let Ok(register_pak) = Register::new(username.to_string(),password.to_string(),email.to_string()){

            match self.stream.write(register_pak.to_bytes().as_slice()) {
                Err(e) => {
                    return Err(e.to_string());
                },
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        return Err("Error con flush".to_string());
                    }
                    return Ok(true);
                }
            }
        }else {
            return Err("Error al crear el paquete de Register".to_string());
        }
    }

    fn votar(&mut self, mut args:Vec<&str>) -> Result<bool,String>{

        if !(args.len() == 2){
            println!("Para votar debe mandar un nombre de nominado y la cantidad de votos");
            return Ok(false);
        }

        let nominado = args.remove(0);
        let cantidad_votos = args.remove(0);

        if let Ok(vote_pak) = Vote::new(nominado.to_string(),cantidad_votos.as_ptr() as u8){

            match self.stream.write(vote_pak.to_bytes().as_slice()) {
                Err(e) => return Err(e.to_string()),
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        return Err("Error con flush".to_string());
                    }
                    return Ok(true);
                } 
            }
        }else {
            return Err("Error al crear el paquete de Register".to_string());
        }
    }

    fn consultar_nominados(&mut self) -> Result<bool,String>{
    
        let mut info_packet = InfoPacket::new(PacketType::from_utf8(6), "Obtener Nominados".to_string());              
        match self.stream.write(info_packet.to_bytes().as_slice()) {
            Err(e) => return Err(e.to_string()),
            Ok(_) => {
                if self.stream.flush().is_err() {
                    return Err("Error con flush".to_string());
                }
                    return Ok(true);
            } 
        }
    }

    pub fn escuchar_respuesta(&mut self) -> Result<(),String>{

        let mut buffer = [0; 1024];
        if let Ok(_size) = self.stream.read(&mut buffer){
            let mut packet = InfoPacket::from_bytes(buffer.to_vec());

            if packet.is_err(){
                return Err(packet.get_msg());
            }

            //aca según lo que retorna el servidor se puede ver si hay que imprimirlo o no por ejemplo

            return Ok(());
        }else{
            return Err("Error al leer respuesta de servidor".to_string());
        }
    }
}