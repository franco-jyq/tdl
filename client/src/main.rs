use cliente::Client;
use std::{io, net::TcpStream, time::Duration};
mod cliente;
use common::colors::{print_common_text, print_error, print_info};

const TIMEOUT_NANO: u32 = 10000000;
const ADDRESS: &str = "127.0.0.1:8095";

fn main() {
    print_info("Conectándome a ", ADDRESS);

    if let Ok(stream) = TcpStream::connect(ADDRESS) {
        inicializar_cliente(stream);
    } else {
        print_error("No se pudo conectar...");
    }
}

fn pause() -> Result<String, String> {
    let mut msg = String::new();

    print_common_text(
        "Escriba que acción quiere realizar o Ayuda para ver los mensajes disponibles",
    );
    match io::stdin().read_line(&mut msg) {
        Ok(_u) => Ok(msg),
        Err(_error) => Err(String::from("Error al leer io")),
    }
}

fn listar_msg() {
    print_common_text("iniciar-sesion [nombre-usuario] [contraseña]");
    print_common_text("registrarse [nombre-usuario] [contraseña mail]");
    print_common_text("consultar-nominados");
    print_common_text("consultar-resultados");
    print_common_text("consultar-saldo");
    print_common_text("votar [nominado] [cantidad]");
    print_common_text("cargar-Saldo [nombre-usuario] [monto]");
    print_common_text("salir");
}

fn inicializar_cliente(stream: TcpStream) {
    stream
        .set_read_timeout(Some(Duration::new(0, TIMEOUT_NANO)))
        .unwrap();

    let mut cliente = Client::new(stream);
    let mut contador = 0;
    loop {
        if let Ok(msg) = pause() {
            let vec_msg: Vec<&str> = msg.split_whitespace().collect();
            if vec_msg.is_empty() {
                print_error("Introduzca algun comando");
                continue;
            }
            let command = vec_msg.first().unwrap().to_lowercase();
            if command == *"ayuda" {
                listar_msg();
                continue;
            } else if command == *"salir" {
                break;
            }

            if contador == 5 {
                std::process::Command::new("clear").status().unwrap();
                listar_msg();
                contador = 0;
            } else {
                contador += 1;
            }
            match cliente.escribir_mensaje(vec_msg) {
                Ok(hay_respuesta) => {
                    if hay_respuesta {
                        if command == *"consultar-nominados" {
                            if let Err(e) = cliente.imprimir_nominados() {
                                println!("{e}");
                                break;
                            }
                            continue;
                        } else if command == *"consultar-resultados" {
                            if let Err(e) = cliente.imprimir_votados() {
                                println!("{e}");
                                break;
                            }
                            continue;
                        }
                        if let Err(e) = cliente.escuchar_respuesta() {
                            println!("{e}");
                            break;
                        }
                        continue;
                    }
                }
                Err(e) => {
                    print_error(&e);
                    break;
                }
            }
        } else {
            return;
        }
    }
}
