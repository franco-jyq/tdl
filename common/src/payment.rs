use crate::{packet_type::PacketType, packet_traits::{UsernameToBytes}};

pub struct Payment {
    packet_type: PacketType,
    //username_size: u8,
    pub username: String,
    pub amount: u32,
}

impl Payment {
    pub fn new(username: String, amount: u32) -> Payment {
        Payment {
            packet_type: PacketType::PAYMENT,
            username,
            amount,
        }
    }

    
    pub fn to_bytes(&mut self) -> Vec<u8> {
        let amount_bytes = self.amount.to_be_bytes().to_vec();
        [self.pkt_type_and_username_to_bytes(),amount_bytes].concat()
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

impl UsernameToBytes for Payment {
    fn get_username (&self) -> String {
        self.username.clone()
    }

    fn get_packet_type(&self) -> PacketType {
        self.packet_type.clone()
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
