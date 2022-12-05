use common::vote::Vote;
use common::{
    colors::{print_common_text, print_cyan, print_error},
    infopacket::InfoPacket,
    login::Login,
    nominees::Nominees,
    packet_traits::ToBytes,
    packet_type::PacketType,
    payment::Payment,
    register::Register,
};
use std::{
    io::{Read, Write},
    str::FromStr,
};

pub struct Client<T: Read + Write> {
    stream: T,
}

impl<T> Client<T>
where
    T: Read + Write,
{
    pub fn new(stream: T) -> Self {
        Client { stream }
    }

    pub fn escribir_mensaje(&mut self, mut vec_msg: Vec<&str>) -> Result<bool, String> {
        let value = vec_msg.remove(0).to_lowercase();

        match value.as_str() {
            "iniciar-sesion" => self.iniciar_sesion(vec_msg),
            "registrarse" => self.registrarse(vec_msg),
            "votar" => self.votar(vec_msg),
            "consultar-votos" => self.consultar_votos(),
            "consultar-nominados" => self.consultar_nominados(),
            "cargar-saldo" => self.cargar_saldo(vec_msg),
            "consultar-saldo" => self.consultar_saldo(),
            _ => {
                print_error(
                    "Nombre de mensaje inválido, ultilize Ayuda para ver los mensajes disponibles",
                );
                Ok(false)
            }
        }
    }

    fn registrarse(&mut self, mut args: Vec<&str>) -> Result<bool, String> {
        if args.len() != 3 {
            print_error("Para registrarse debe mandar un nombre de usuario,contraseña y mail");
            return Ok(false);
        }

        let username = args.remove(0);
        let password = args.remove(0);
        let email = args.remove(0);

        if let Ok(register_packet) = Register::new(
            username.to_string(),
            password.to_string(),
            email.to_string(),
        ) {
            self.enviar_mensaje(register_packet)
        } else {
            Err("Error al crear el paquete de Register".to_string())
        }
    }

    fn iniciar_sesion(&mut self, mut args: Vec<&str>) -> Result<bool, String> {
        if args.len() != 2 {
            print_error("Para iniciar sesion debe mandar un nombre de usuario,contraseña y mail");
            return Ok(false);
        }

        let username = args.remove(0);
        let password = args.remove(0);

        if let Ok(login_packet) = Login::new(username.to_string(), password.to_string()) {
            self.enviar_mensaje(login_packet)
        } else {
            Err("Error al crear el paquete de Login".to_string())
        }
    }

    fn votar(&mut self, mut args: Vec<&str>) -> Result<bool, String> {
        if args.len() != 2 {
            print_error("Para votar debe mandar un nombre de nominado y la cantidad de votos");
            return Ok(false);
        }

        let nominado = args.remove(0);
        let cantidad_votos = u8::from_str(args.remove(0)).unwrap();
        println!("{}", cantidad_votos);
        if let Ok(vote_packet) = Vote::new(nominado.to_string(), cantidad_votos) {
            self.enviar_mensaje(vote_packet)
        } else {
            Err("Error al crear el paquete de Register".to_string())
        }
    }

    fn consultar_nominados(&mut self) -> Result<bool, String> {
        let info_packet =
            InfoPacket::new(PacketType::from_utf8(6), "Obtener Nominados".to_string());
        self.enviar_mensaje(info_packet)
    }

    fn consultar_votos(&mut self) -> Result<bool, String> {
        let info_packet = InfoPacket::new(PacketType::from_utf8(6), "Obtener Votos".to_string());
        self.enviar_mensaje(info_packet)
    }

    fn consultar_saldo(&mut self) -> Result<bool, String> {
        let info_packet = InfoPacket::new(PacketType::from_utf8(6), "Obtener Saldo".to_string());
        self.enviar_mensaje(info_packet)
    }

    fn cargar_saldo(&mut self, mut args: Vec<&str>) -> Result<bool, String> {
        if args.len() != 2 {
            print_error("Para cargar saldo debe mandar el monto a cargar");
            return Ok(false);
        }

        let username = args.remove(0);
        let monto = args.remove(0);

        let payment = Payment::new(username.to_string(), FromStr::from_str(monto).unwrap());
        self.enviar_mensaje(payment)
    }

    pub fn escuchar_respuesta(&mut self) -> Result<(), String> {
        let mut buffer = [0; 1024];
        if let Ok(_size) = self.stream.read(&mut buffer) {
            let aux = buffer[0];
            let first_byte = PacketType::from_utf8(aux);
            match first_byte {
                PacketType::INFO | PacketType::ERROR => {
                    let mut packet = InfoPacket::from_bytes(buffer.to_vec());

                    //if packet.is_err(){
                    //    return Err(packet.get_msg());
                    //}
                    println!("{}", packet.get_msg());
                    //aca según lo que retorna el servidor se puede ver si hay que imprimirlo o no por ejemplo
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Err("Error al leer respuesta de servidor".to_string())
        }
    }

    pub fn imprimir_nominados(&mut self) -> Result<(), String> {
        let mut buffer = [0; 1024];
        if let Ok(_size) = self.stream.read(&mut buffer) {
            let aux = buffer[0];
            let first_byte = PacketType::from_utf8(aux);
            match first_byte {
                PacketType::NOMINEES => {
                    let nominees = Nominees::from_bytes(buffer.to_vec());

                    print_cyan("Los Nominados Son:");

                    for n in nominees.nominees.iter() {
                        print_common_text(common::nominees::get_name(n).as_str());
                    }
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Err("Error al leer respuesta de servidor".to_string())
        }
    }

    pub fn imprimir_votados(&mut self) -> Result<(), String> {
        let mut buffer = [0; 1024];
        if let Ok(_size) = self.stream.read(&mut buffer) {
            let aux = buffer[0];
            let first_byte = PacketType::from_utf8(aux);
            match first_byte {
                PacketType::INFO => {
                    let mut votados = InfoPacket::from_bytes(buffer.to_vec());

                    print_cyan("Los Votos Son:");
                    for v in votados.get_msg().split(';'){
                        println!("{}",v);
                    }
                    
                    Ok(())
                }
                _ => Ok(()),
            }
        } else {
            Err("Error al leer respuesta de servidor".to_string())
        }
    }

    fn enviar_mensaje<U: ToBytes>(&mut self, u: U) -> Result<bool, String> {
        match self.stream.write(u.to_bytes().as_slice()) {
            Err(e) => Err(e.to_string()),
            Ok(_) => {
                if self.stream.flush().is_err() {
                    return Err("Error con flush".to_string());
                }
                Ok(true)
            }
        }
    }
}

#[cfg(test)]

mod cliente_tests {

    use super::*;
    use std::io;

    struct MockTcpStream {
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        /// Lee bytes del stream hasta completar el buffer y devuelve cuantos bytes fueron escritos en el stream
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            self.write_data.as_slice().read(buf)
        }
    }

    impl Write for MockTcpStream {
        /// Escribe el valor del buffer en el stream y devuelve cuantos bytes fueron escritos
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.write_data.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.write_data.flush()
        }
    }

    /// El mensaje se envia correctamente a travez del stream
    #[test]
    fn register_test() {
        let stream = MockTcpStream { write_data: vec![] };
        let mut client = Client { stream: stream };
        let bytes: &[u8] = &Register::new(
            "Frodo".to_string(),
            "aguante_milei".to_string(),
            "frodoneta@gmail.com".to_string(),
        )
        .unwrap()
        .to_bytes();

        client
            .escribir_mensaje(vec![
                "Registrarse",
                "Frodo",
                "aguante_milei",
                "frodoneta@gmail.com",
            ])
            .unwrap();

        let mut buf = [0; 500];
        client.stream.read(&mut buf).unwrap();
        assert_eq!(client.stream.write_data, bytes);
    }
}
