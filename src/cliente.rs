use std::{env::args, io::Write, net::TcpStream}; //esto es para probar el codigo
static CLIENT_ARGS: usize = 3;
use std::io;

fn main() {
    //let _socket =  SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0 , 0, 1)), 80);
    let argv = args().collect::<Vec<String>>();
    if argv.len() != CLIENT_ARGS {
        println!("Cantidad de argumentos inválido");
        let app_name = &argv[0];
        println!("{:?} <host> <puerto>", app_name);
    }

    let address = argv[1].clone() + ":" + &argv[2];
    println!("Conectándome a {:?}", address);

    if let Ok(mut stream) = TcpStream::connect(address) {
        println!("Connectado al servidor!");
        loop {
            escribir_mensaje(&mut stream);
        }
    } else {
        println!("No se pudo conectar...");
    }
}

fn escribir_mensaje(stream: &mut TcpStream) {
    let mut command: String = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    command.remove(command.len() - 1);
    command.push('\r');
    command.push('\n');
    let mensaje = command.clone();
    match stream.write(mensaje.as_bytes()) {
        Err(_) => println!("Fallo conexion con servidor"),
        Ok(_) => {
            if stream.flush().is_err() {
                println!("Error con flush")
            }
        }
    }
}
