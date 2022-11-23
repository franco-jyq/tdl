use std::{io::{Write}, net::TcpStream, sync::mpsc::{Sender,Receiver}};
use common::register::Register;

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

    pub fn escribir_mensaje(&mut self,mut vec_msg:Vec<&str>) {
        
        let value = vec_msg.remove(0);

        match value{
            "Iniciar-Sesion" => (),
            "Registrarse" => self.registrarse(vec_msg),
            "Votar" => (),
            "Consultar-Votos" => (),
            "Consultar-Nominados" => (),
            "Salir" => self.salir(),
            _ => return,
        }
    }

    fn registrarse(&mut self,mut args:Vec<&str>){

        if !(args.len() == 3){
            println!("Para registrarse debe mandar un nombre de usuario,contraseña y mail");
            return;
        }

        let username = args.remove(0);
        let password = args.remove(0);
        let email = args.remove(0);

        if let Ok(mut register_pak) = Register::new(username.to_string(),password.to_string(),email.to_string()){

            match self.stream.write(register_pak.to_bytes().as_slice()) {
                Err(_) => println!("Fallo conexion con servidor"),
                Ok(_) => {
                    if self.stream.flush().is_err() {
                        println!("Error con flush")
                    }
                }
            }
        }
    }

    fn salir(&mut self){
        self.tx.send("Salir".to_string()).unwrap();
    }

}