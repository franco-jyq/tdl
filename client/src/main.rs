use std::{env::args, net::TcpStream};
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

    if let Ok(stream) = TcpStream::connect(address) {
        println!("Connectado al servidor!");
        let mut cliente = Client::new(stream);
        loop {
            cliente.escribir_mensaje();
        }
    } else {
        println!("No se pudo conectar...");
    }

}


