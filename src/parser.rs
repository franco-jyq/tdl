use crate::vote::*;
use crate::pay::*;


#[derive(Debug)]
pub enum Message {
    Vote(VoteInfo),
    Pay(PayInfo),
}

pub struct MessageCommand {
    pub cmd: Message,
}

fn set_indice(comando_filtrado: &str) -> usize {
    let mut indice_comando: usize = 1;
    if comando_filtrado.as_bytes()[0] != b':' {
        indice_comando = 0;
    }
    indice_comando
}

fn limpiar_final(cmd: String) -> String {
    let mut nuevo = cmd;
    let len = nuevo.len();
    for _ in 0..2 {
        nuevo.remove(len - 2);
    }
    nuevo
}

fn separar_parametros(indice_comando: usize, message_split: &[&str]) -> Vec<String> {
    let mut parametros: Vec<String> = Vec::new();
    for indice in message_split.iter().skip(indice_comando + 1) {
        parametros.push(indice.to_string());
    }
    parametros
}
pub fn parser(cmd: String) -> Result<MessageCommand, String> {
    // println!("/{}/",cmd);
    if cmd.len() == 2 {
        return Err(String::from("no se escribio ningun comando"));
    }
    let _last_two: Vec<char> = cmd.chars().rev().take(2).collect();

    let nuevo_comando = limpiar_final(cmd);
    let comando_filtrado = nuevo_comando.trim();
    let indice_comando = set_indice(comando_filtrado);

    let split = comando_filtrado.split(' ');
    let message_split = split.collect::<Vec<&str>>();
    let comando = message_split[indice_comando];

    let parametros = separar_parametros(indice_comando, &message_split);

    let mensaje: Message;
    match comando {
        "VOTE" => mensaje = Message::Vote(VoteInfo::new(parametros)?),
        "PAY" => mensaje = Message::Pay(PayInfo::new(parametros)?),
        _ => return Err(String::from("No se identifico el Comando")),
    }

    /*init message command*/
    let prefix: String;
    match indice_comando {
        0 => prefix = String::from("-1"),
        1 => {
            message_split[0].to_string().remove(0);
            prefix = message_split[0].to_string();
        }
        _ => return Err(String::from("No se identifico el indice del comando")),
    }
    let message_cmd = MessageCommand {
        cmd: mensaje,
    };
    Ok(message_cmd)
}
