use std::{env::args, io, sync::mpsc::{Sender,Receiver, self}, thread, net::TcpStream, time::Duration};
static CLIENT_ARGS: usize = 3;
use cliente::Client;
use listener::Listener;
mod cliente;
mod listener;

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
    let stream_cpy = stream.try_clone().unwrap();

    let (tx_1, rx_1): (Sender<String>,Receiver<String>) = mpsc::channel();
    let (tx_2, rx_2): (Sender<String>,Receiver<String>) = mpsc::channel();

    let mut listener = Listener::new(tx_2,rx_1,stream_cpy);
    let handle = thread::spawn(move || {
        listener.escuchar_server();
    });

    let mut cliente = Client::new(stream,tx_1,rx_2);
    loop {
        if let Ok(msg) = pause(){
            //Parseo en un vector la linea leida
            let vec_msg:Vec<&str> = msg.split_whitespace().collect();

            if vec_msg.get(0).unwrap() == &"Ayuda"{
                listar_msg();
                continue;
            }else if vec_msg.get(0).unwrap() == &"Salir"{
                cliente.escribir_mensaje(vec_msg);
                break;
            }

            cliente.escribir_mensaje(vec_msg);

        }else{
            return
        }
    }
    
    handle.join().unwrap();
}
