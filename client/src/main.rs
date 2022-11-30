use std::{env::args, io, net::TcpStream, time::Duration};
static CLIENT_ARGS: usize = 3;
use cliente::Client;
mod cliente;
use common::colors::{print_error, print_info, print_common_text};

const TIMEOUT_NANO:u32 = 10000000;

fn main() {
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        print_error("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
        return;
    }

    let address = argv[1].clone() + ":" + &argv[2];
    print_info("Conectándome a ", &address);

    if let Ok(stream) = TcpStream::connect(address) {
        inicializar_cliente(stream);
    }else{
        print_error("No se pudo conectar...");
    }

}

//Lee el input para obtener el tipo de mensaje a enviar.
fn pause() -> Result<String,String>{
    
    let mut msg = String::new();

    print_common_text("Escriba que acción quiere realizar o Ayuda para ver los mensajes disponibles");
    match io::stdin().read_line(&mut msg) {
        Ok(_u) => {
            Ok(msg)
        }
        Err(_error) => Err(String::from("Error al leer io"))
    }

}

fn listar_msg(){
    print_common_text("Iniciar-Sesion nombre-usuario contraseña");
    print_common_text("Registrarse nombre-usuario contraseña mail");
    print_common_text("Consultar-Nominados");
    print_common_text("Consultar-Votos");
    print_common_text("Votar nominado");
    print_common_text("Cargar-Saldo saldo");
    print_common_text("Salir");
}

fn inicializar_cliente(stream:TcpStream){

    stream.set_read_timeout(Some(Duration::new(0, TIMEOUT_NANO))).unwrap();

    let mut cliente = Client::new(stream);
    loop {
        if let Ok(msg) = pause(){
            //Parseo en un vector la linea leida
            let vec_msg:Vec<&str> = msg.split_whitespace().collect();

            if vec_msg.first().unwrap() == &"Ayuda"{
                listar_msg();
                continue;
            }else if vec_msg.first().unwrap() == &"Salir"{
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
                    print_error(&e);
                    break
                }
            }
        }else{
            return
        }
    }
    
}
