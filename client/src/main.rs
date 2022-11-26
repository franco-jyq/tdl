use std::{env::args, io, net::TcpStream, time::Duration};
static CLIENT_ARGS: usize = 3;
use cliente::Client;
mod cliente;

const TIMEOUT_NANO:u32 = 10000000;

fn main() {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
    }

    let address = argv[1].clone() + ":" + &argv[2];
    println!("Conectándome a {:?}", address);

    if let Ok(stream) = TcpStream::connect(address) {
        inicializar_cliente(stream);
    }else{
        println!("No se pudo conectar...");
        return;
    }

}

//Lee el input para obtener el tipo de mensaje a enviar.
fn pause() -> Result<String,String>{
    
    let mut msg = String::new();
    
    println!("Escriba que acción quiere realizar o Ayuda para ver los mensajes disponibles");
    match io::stdin().read_line(&mut msg) {
        Ok(_u) => {
            return Ok(msg);
        }
        Err(_error) => return Err(String::from("Error al leer io")),
    }

}

fn listar_msg(){
    println!("Inicial-Sesion nombre-usuario contraseña");
    println!("Registrarse nombre-usuario contraseña mail");
    println!("Consultar-Nominados");
    println!("Consultar-Votos");
    println!("Votar nominado");
    println!("Salir");
}

fn inicializar_cliente(stream:TcpStream){

    stream.set_read_timeout(Some(Duration::new(0, TIMEOUT_NANO))).unwrap();

    let mut cliente = Client::new(stream);
    loop {
        if let Ok(msg) = pause(){
            //Parseo en un vector la linea leida
            let vec_msg:Vec<&str> = msg.split_whitespace().collect();

            if vec_msg.get(0).unwrap() == &"Ayuda"{
                listar_msg();
                continue;
            }else if vec_msg.get(0).unwrap() == &"Salir"{
                break;
            }

            match cliente.escribir_mensaje(vec_msg){
                Ok(hay_respuesta) => {
                    if hay_respuesta{
                        match cliente.escuchar_respuesta(){
                            Ok(_) => continue,
                            Err(e) => {
                                println!("{e}");
                                break
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("{e}");
                    break
                }
            }
        }else{
            return
        }
    }
    
}
