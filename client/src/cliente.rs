use std::{io::{Write, Read}, net::TcpStream};
use common::{register::Register, packet_type::PacketType, infopacket::InfoPacket, login::Login, colors::print_error};
use common::vote::Vote;


pub struct Client {
        stream: TcpStream
}

impl Client {
    
    pub fn new(stream:TcpStream) -> Self {

        Client {
            stream
        }
    }

    pub fn escribir_mensaje(&mut self,mut vec_msg:Vec<&str>) -> Result<bool,String>{
        
        let value = vec_msg.remove(0);

        match value{
            "Iniciar-Sesion" => self.iniciar_sesion(vec_msg),
            "Registrarse" => self.registrarse(vec_msg),
            "Votar" => self.votar(vec_msg),
            "Consultar-Votos" => Ok(false),
            "Consultar-Nominados" => self.consultar_nominados(),
            _ => {
                print_error("Nombre de mensaje inválido, ultilize Ayuda para ver los mensajes disponibles");
                Ok(false)},
        }
    }

    fn registrarse(&mut self,mut args:Vec<&str>) -> Result<bool,String>{

        if args.len() != 3 {
            print_error("Para registrarse debe mandar un nombre de usuario,contraseña y mail");
            return Ok(false);
        }

        let username = args.remove(0);
        let password = args.remove(0);
        let email = args.remove(0);

        if let Ok(register_pak) = Register::new(username.to_string(),password.to_string(),email.to_string()){

            match self.stream.write(register_pak.to_bytes().as_slice()) {
                Err(e) => {
                    Err(e.to_string())
                },
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        return Err("Error con flush".to_string());
                    }
                    Ok(true)
                }
            }
        }else {
            Err("Error al crear el paquete de Register".to_string())
        }
    }

    fn iniciar_sesion(&mut self,mut args:Vec<&str>) -> Result<bool,String>{

        if args.len() != 2 {
            print_error("Para iniciar sesion debe mandar un nombre de usuario,contraseña y mail");
            return Ok(false);
        }

        let username = args.remove(0);
        let password = args.remove(0);

        if let Ok(login_packet) = Login::new(username.to_string(),password.to_string()){

            match self.stream.write(login_packet.to_bytes().as_slice()) {
                Err(e) => {
                    Err(e.to_string())
                },
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        return Err("Error con flush".to_string());
                    }
                    Ok(true)
                }
            }
        }else {
            Err("Error al crear el paquete de Login".to_string())
        }
    }

    fn votar(&mut self, mut args:Vec<&str>) -> Result<bool,String>{

        if args.len() != 2 {
            print_error("Para votar debe mandar un nombre de nominado y la cantidad de votos");
            return Ok(false);
        }

        let nominado = args.remove(0);
        let cantidad_votos = args.remove(0);

        if let Ok(vote_pak) = Vote::new(nominado.to_string(),cantidad_votos.as_ptr() as u8){

            match self.stream.write(vote_pak.to_bytes().as_slice()) {
                Err(e) => Err(e.to_string()),
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        return Err("Error con flush".to_string());
                    }
                    Ok(true)
                } 
            }
        }else {
            Err("Error al crear el paquete de Register".to_string())
        }
    }

    fn consultar_nominados(&mut self) -> Result<bool,String>{
    
        let mut info_packet = InfoPacket::new(PacketType::from_utf8(6), "Obtener Nominados".to_string());              
        match self.stream.write(info_packet.to_bytes().as_slice()) {
            Err(e) => Err(e.to_string()),
            Ok(_) => {
                if self.stream.flush().is_err() {
                    return Err("Error con flush".to_string());
                }
                    Ok(true)
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
            Ok(())
        }else{
            Err("Error al leer respuesta de servidor".to_string())
        }
    }
}