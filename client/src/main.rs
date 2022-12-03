use std::{io, net::TcpStream, time::Duration};
use cliente::Client;
mod cliente;
use common::colors::{print_error, print_info, print_common_text};

const TIMEOUT_NANO:u32 = 10000000;
const ADDRESS:&str = "127.0.0.1:8095";

fn main() {
    print_info("Conectándome a ", ADDRESS);

    if let Ok(stream) = TcpStream::connect(ADDRESS) {
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
    print_common_text("iniciar-sesion nombre-usuario contraseña");
    print_common_text("registrarse nombre-usuario contraseña mail");
    print_common_text("consultar-nominados");
    print_common_text("consultar-votos");
    print_common_text("votar nominado");
    print_common_text("cargar-Saldo nombre-usuario monto");
    print_common_text("salir");
}

fn inicializar_cliente(stream:TcpStream){

    stream.set_read_timeout(Some(Duration::new(0, TIMEOUT_NANO))).unwrap();

    let mut cliente = Client::new(stream);
    let mut contador = 0;
    loop {
        if let Ok(msg) = pause(){
            //Parseo en un vector la linea leida
            let vec_msg:Vec<&str> = msg.split_whitespace().collect();

            let command = vec_msg.first().unwrap().to_lowercase();
            if command == "ayuda".to_string(){
                listar_msg();
                continue;
            }else if command == "salir".to_string(){
                break;
            }

            if contador == 5{
                std::process::Command::new("clear").status().unwrap();
                listar_msg();
                contador = 0;
            }else {
                contador += 1;
            }
            match cliente.escribir_mensaje(vec_msg){
                Ok(hay_respuesta) => {
                    if hay_respuesta{

                        if command == "consultar-nominados".to_string(){
                            match cliente.imprimir_nominados(){
                                Ok(_) => continue,
                                Err(e) => {
                                    println!("{e}");
                                    break
                                }
                            }
                        }
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
