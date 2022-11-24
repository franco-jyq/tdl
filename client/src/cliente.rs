use std::{io::{Write}, net::TcpStream, sync::mpsc::{Sender,Receiver}};
use common::register::Register;
use common::vote::Vote;

pub struct Client {
        stream: TcpStream,
        tx: Sender<String>,
        rx: Receiver<String>
}

impl Client {
    
    pub fn new(stream:TcpStream,tx:Sender<String>,rx:Receiver<String>) -> Self {

        Client {
            stream : stream,
            tx: tx,
            rx: rx
        }
    }

    pub fn escribir_mensaje(&mut self,mut vec_msg:Vec<&str>) -> Result<(),String>{
        
        let value = vec_msg.remove(0);

        match value{
            "Iniciar-Sesion" => (),
            "Registrarse" => return self.registrarse(vec_msg),
            "Votar" => return self.votar(vec_msg),
            "Consultar-Votos" => (),
            "Consultar-Nominados" => (),
            "Salir" => self.salir(),
            _ => return {
                println!("Nombre de mensaje inválido, ultilize Ayuda para ver los mensajes disponibles");
                Ok(())},
        }
        Ok(())
    }

    fn registrarse(&mut self,mut args:Vec<&str>) -> Result<(),String>{

        if !(args.len() == 3){
            println!("Para registrarse debe mandar un nombre de usuario,contraseña y mail");
            return Ok(());
        }

        let username = args.remove(0);
        let password = args.remove(0);
        let email = args.remove(0);

        if let Ok(register_pak) = Register::new(username.to_string(),password.to_string(),email.to_string()){

            match self.stream.write(register_pak.to_bytes().as_slice()) {
                Err(e) => {
                    self.salir();
                    return Err(e.to_string());
                },
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        self.salir();
                        return Err("Error con flush".to_string());
                    }
                    return Ok(());
                }
            }
        }else {
            return Err("Error al crear el paquete de Register".to_string());
        }
    }

    fn votar(&mut self, mut args:Vec<&str>) -> Result<(),String>{

        if !(args.len() == 2){
            println!("Para votar debe mandar un nombre de nominado y la cantidad de votos");
            return Ok(());
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
                    return Ok(());
                } 
            }
        }else {
            return Err("Error al crear el paquete de Register".to_string());
        }
    }

    fn salir(&mut self){
        self.tx.send("Salir".to_string()).unwrap();
    }

    pub fn escuchar_listener(&mut self) {

        //La idea es ver si hay algo para leer para que no se bloquee, aprovechando el uso del listener
        //el cliente no se bloquea en caso de que el servidor tarde en responder o si el servidor mandará 
        //un mensaje sin enviarle una consulta.

        if let Ok(leido) = self.rx.try_recv(){
            println!("Escucho del listener: {leido}");
        }
    }
}