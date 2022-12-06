use crate::{packet_traits::ToBytes, packet_type::PacketType};

static MAX_NOMINADO_SIZE: u8 = 255;

#[derive(Debug)]
pub struct Vote {
    packet_type: PacketType,
    nominado_size: u8,
    pub nominado: String,
    pub cantidad_votos: u8,
}

impl Vote {
    pub fn new(nominado: String, cantidad_votos: u8) -> Result<Vote, String> {
        let nominado_size = nominado.len() as u8;

        if nominado_size > MAX_NOMINADO_SIZE {
            return Err(String::from("INVALID_NOMINADO_SIZE"));
        }

        Ok(Vote {
            packet_type: PacketType::Vote,
            nominado_size,
            nominado,
            cantidad_votos,
        })
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Vote {
        let mut i = 0; // Contador

        let nominado_size = bytes[1] as usize;
        i += nominado_size + 2;
        let nominado = String::from_utf8(bytes[2..i].to_vec()).unwrap();

        let cantidad_votos = bytes[i];

        Vote::new(nominado, cantidad_votos).unwrap()
    }
}

impl ToBytes for Vote {
    fn to_bytes(&self) -> Vec<u8> {
        let packet_type_bytes = self.packet_type.as_utf8().to_be_bytes().to_vec();
        let nominado_size_bytes = self.nominado_size.to_be_bytes().to_vec();
        let nominado_bytes = self.nominado.as_bytes().to_vec();
        let cantidad_votos_bytes = self.cantidad_votos.to_be_bytes().to_vec();
        [
            packet_type_bytes,
            nominado_size_bytes,
            nominado_bytes,
            cantidad_votos_bytes,
        ]
        .concat()
    }
}

#[cfg(test)]

mod vote_tests {

    use crate::packet_traits::ToBytes;

    use super::Vote;

    #[test]
    fn vote_to_bytes_test() {
        let test_packet = Vote::new("user".to_string(), 2).unwrap();
        let expected = vec![4, 4, 117, 115, 101, 114, 2];
        assert_eq!(test_packet.to_bytes(), expected);
    }

    #[test]
    fn vote_from_bytes_test() {
        let expected_packet = Vote::new("user".to_string(), 2).unwrap();
        let bytes = vec![4, 4, 117, 115, 101, 114, 2];
        let test_packet = Vote::from_bytes(bytes);
        assert_eq!(test_packet.cantidad_votos, expected_packet.cantidad_votos);
    }
}
