use crate::packet_type::PacketType;

pub struct Payment {
    packet_type: PacketType,
    username_size: u8,
    pub username: String,
    pub amount: u32,
}

impl Payment {
    pub fn new(username: String, amount: u32) -> Payment {
        let username_size = username.len() as u8;
        Payment {
            packet_type: PacketType::PAYMENT,
            username_size,
            username,
            amount,
        }
    }

    // Comentario personal: Ver el tema traits para estos dos metodos,
    // ya que todos los paquetes comparten la misma firma
    pub fn to_bytes(&mut self) -> Vec<u8> {
        let packet_type_bytes = self.packet_type.as_utf8().to_be_bytes().to_vec();
        let username_size_bytes = self.username_size.to_be_bytes().to_vec();
        let username_bytes = self.username.as_bytes().to_vec();
        let amount_bytes = self.amount.to_be_bytes().to_vec();
        [
            packet_type_bytes,
            username_size_bytes,
            username_bytes,
            amount_bytes,
        ]
        .concat()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Payment {
        let mut i = 0; // Contador

        let username_size = bytes[1] as usize;
        i += username_size + 2;
        let username = String::from_utf8(bytes[2..i].to_vec()).unwrap(); //Estos unwraps...
        let amount = u32::from_be_bytes(bytes[i..i + 4].try_into().unwrap());

        Payment::new(username, amount)
    }
}

#[cfg(test)]

mod payment_test {
    use super::Payment;

    #[test]
    fn payment_to_bytes_test() {
        let mut test_packet = Payment::new("user".to_string(), 100 as u32);

        let expected = vec![2, 4, 117, 115, 101, 114, 0, 0, 0, 100];
        assert_eq!(test_packet.to_bytes(), expected);
    }

    #[test]
    fn payment_from_bytes_test() {
        let bytes = vec![2, 4, 117, 115, 101, 114, 0, 0, 0, 15];

        let pkt = Payment::from_bytes(bytes);
        assert_eq!(pkt.username, "user".to_string());
        assert_eq!(pkt.amount, 15)
    }
}
