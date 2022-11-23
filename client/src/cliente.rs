use std::{io::{Write}, net::TcpStream, thread,time::Duration, sync::mpsc::{Sender,Receiver, self}};
use common::register::Register;


const PACKET_SIZE:usize = 10;
const TIMEOUT_NANO:u32 = 10000000;
pub struct Client {
        stream: TcpStream,
        tx: Sender<String>
}

impl Client {
    
    pub fn new(address:String) -> Result<Self, ()> {
        
        if let Ok(stream) = TcpStream::connect(address) {
            println!("Connectado al servidor!");

            let (tx, rx): (Sender<String>,Receiver<String>) = mpsc::channel();

            stream.set_read_timeout(Some(Duration::new(0, TIMEOUT_NANO))).unwrap();
            let stream_cpy = stream.try_clone().unwrap();
            thread::spawn(|| {
                escuchar_server(stream_cpy,rx)
            });

            return Ok(Client {
                stream : stream,
                tx: tx
            })
        }else {
            println!("No se pudo conectar...");
            return Err(())
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

fn escuchar_server(_stream:TcpStream, rx:Receiver<String>){
    loop{
        //let mut buf = [0;PACKET_SIZE];

        //let _leido_server = stream.read(&mut buf).unwrap();

        if  let Ok(leido_cliente) = rx.recv_timeout(Duration::new(0, TIMEOUT_NANO)){
            println!("{leido_cliente}"); //si no se joinea nunca se va a ver
            break; 
        }
    }
}