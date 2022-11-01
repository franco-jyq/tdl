use std::{net::{TcpListener, TcpStream},io::{BufReader, BufRead}, thread};
static PORT: &str = "127.0.0.1:8095";
use gh22::parser::*;


fn main() {
    start_server().unwrap();
}


fn start_server() -> Result<(), String>{
    // Creo que estaria bien dejar elegir al puerto para configurar el server
    // Reservemos el pattern matching para cuando hay mas de 2, asi mostramos varias formas
    if let Ok(listener) = TcpListener::bind(PORT) {        
        obtain_connections(listener)?
    }
    Err(String::from("Error inicializando servidor"))
}


fn obtain_connections(listener: TcpListener) -> Result<(), String> {
    //Obtenemos las conexiones establecidas
    //launch_server_main_thread()?;
    launch_client_handler_threads(listener)?;
    Ok(())
    // El servidor esta a la espera de que un cliente se conecte, entonces primero hay que leer
}



fn launch_client_handler_threads(
    listener: TcpListener,
) -> Result<(), String> {
    for client in listener.incoming().flatten() {
        let _join_handle = thread::spawn(move || {
            handle_client(client).ok();             
        });      
        // Err(String::from("Error con el cliente")); // Creo que en error simplemente deberia continuar
    }
    Ok(()) 
}

/* Esta función honestamente no se para que esta
fn launch_server_main_thread(

) -> Result<(), String> {
   
    thread::spawn(move || {
        println!("main server thred");

    });

    Ok(())
} */

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
