use std::{net::{TcpListener, TcpStream}, sync::{Arc, RwLock}, collections::HashMap, io::{BufReader, BufRead}, thread};
static PORT: &str = "127.0.0.1:8095";
use gh22::parser::*;


fn main() {
    manage_server().unwrap();
}


fn manage_server() -> Result<(), String>{

        match TcpListener::bind(PORT) {
            Err(_) => return Err(String::from("Error con el bind")),
            Ok(listener) => obtain_connections(listener)?,
        }
        Ok(())

}

fn obtain_connections(listener: TcpListener) -> Result<(), String> {
    //Obtenemos las conexiones establecidas
    launch_server_main_thread()?;
    launch_client_handler_threads(listener)?;
    Ok(())
    // El servidor esta a la espera de que un cliente se conecte, entonces primero hay que leer
}



fn launch_client_handler_threads(
    listener: TcpListener,
) -> Result<(), String> {
    for cliente in listener.incoming() {

        let _join_handle: thread::JoinHandle<_> = thread::spawn(move || {
            match cliente {
                // mover el match para arriba de todo
                Err(_) => return Err(String::from("Error con el cliente")),
                Ok(cliente_tcp) => handle_client(cliente_tcp)?,
            };
            Ok(())
        });
    }
    Ok(())
}

fn launch_server_main_thread(

) -> Result<(), String> {
   
    thread::spawn(move || {
        println!("main server thred");

    });

    Ok(())
}

fn handle_client(
    cliente: TcpStream
) -> Result<(), String> {
    
    let reader = BufReader::new(cliente);

    // CASO DE QUE CLIENTE NO ESTE REGISTRADO
    for line in reader.lines() {
        match line {
            Err(_) => return Err(String::from("Error con el line del cliente")),
            Ok(linea_no_parseada) => match parser(linea_no_parseada.to_string()) {
                Err(error) => println!("error: {}", error),
                Ok(message) => match match_message(message) {
                    Err(error) => println!("error: {}", error),
                    Ok(_) => println!("F"),
                },
            },
        }
    }
    Ok(())
}



fn match_message(
    message: MessageCommand
) -> Result<(), String> {
    // println!("EN matchear mensaje llega :{:?}",data_user);
    match message.cmd {
        Message::Vote(vote_info) => println!("{:?}", vote_info),
        Message::Pay(pay_info) => println!("{:?}", pay_info),
    }
    Ok(())
}
