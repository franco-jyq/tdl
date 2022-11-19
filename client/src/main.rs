use std::{env::args, io};
static CLIENT_ARGS: usize = 3;
use cliente::Client;
mod cliente;

fn main() {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
    }

    let address = argv[1].clone() + ":" + &argv[2];
    println!("Conectándome a {:?}", address);


    if let Ok(mut cliente) = Client::new(address){
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

                cliente.escribir_mensaje(vec_msg);

            }else{
                return
            }
        }
    }
}

//Lee el input para obtener el tipo de mensaje a enviar.
fn pause() -> Result<String,String>{

    let mut msg = String::new();
    
    println!("Escriba que acción quiere realizar o Ayuda para ver los mensajes disponibles");
    match io::stdin().read_line(&mut msg) {
        Ok(n) => {
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
