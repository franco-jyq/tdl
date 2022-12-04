use crate::{packet_traits::ToBytes, packet_type::PacketType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nominee {
    name_size: u8,
    name: String,
}

impl Nominee {
    pub fn to_bytes(&self) -> Vec<u8> {
        let name_size_bytes = self.name_size.to_be_bytes().to_vec();
        let name_bytes = self.name.as_bytes().to_vec();
        [name_size_bytes, name_bytes].concat()
    }
}

//#[derive(Debug, Clone)]
pub struct Nominees {
    packet_type: PacketType,
    nominees_size: u32, // Tamaño en bytes del vector de abajo
    pub nominees: Vec<Nominee>,
}

impl Nominees {
    pub fn new(names: Vec<String>) -> Nominees {
        let mut new_nominees = Nominees {
            packet_type: PacketType::NOMINEES,
            nominees_size: 0,
            nominees: vec![],
        };
        for name in names.iter() {
            // Buscar forma de iterar que consuma el vector
            new_nominees.nominees.push(Nominee {
                name_size: name.len() as u8,
                name: name.clone(),
            });
            new_nominees.nominees_size += name.len() as u32 + 1;
        }
        new_nominees
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Nominees {
        let nominees_size = u32::from_be_bytes(bytes[1..5].try_into().unwrap());
        let mut i = 5; // Contador
        let mut names = vec![];
        while i < 5 + (nominees_size as usize) {
            let name_lenght = bytes[i];
            let name =
                String::from_utf8(bytes[i + 1..i + 1 + name_lenght as usize].to_vec()).unwrap();
            names.push(name);
            i += 1 + name_lenght as usize;
        }

        Nominees::new(names)
    }
}

impl ToBytes for Nominees {
    fn to_bytes(&self) -> Vec<u8> {
        let mut result = vec![];
        result.push(self.packet_type.as_utf8().to_be_bytes().to_vec());
        result.push(self.nominees_size.to_be_bytes().to_vec());
        for nominee in self.nominees.iter() {
            result.push(nominee.to_bytes())
        }
        result.concat()
    }
}

pub fn get_name(nominee: &Nominee) -> String {
    nominee.name.clone()
}

#[cfg(test)]

mod nominees_test {
    use std::vec;

    use crate::packet_traits::ToBytes;

    use super::Nominees;

    #[test]
    fn nominees_to_bytes_test() {
        let names = vec!["juan".to_string(), "franco".to_string()];
        let test_packet = Nominees::new(names);

        let expected = vec![
            7, 0, 0, 0, 12, 4, 106, 117, 97, 110, 6, 102, 114, 97, 110, 99, 111,
        ];
        assert_eq!(test_packet.to_bytes(), expected);
    }

    #[test]
    fn nominees_from_bytes_test() {
        let names = vec!["juan".to_string(), "axel".to_string()];
        let expected_packet = Nominees::new(names);

        let bytes = vec![7, 0, 0, 0, 10, 4, 106, 117, 97, 110, 4, 97, 120, 101, 108];
        let pkt = Nominees::from_bytes(bytes);
        assert_eq!(pkt.nominees, expected_packet.nominees);
    }

    #[test]
    fn nominees_from_bytes_test2() {
        let names = vec!["juan".to_string(), "franco".to_string(), "axel".to_string()];
        let expected_packet = Nominees::new(names);

        let bytes = vec![
            7, 0, 0, 0, 16, 4, 106, 117, 97, 110, 6, 102, 114, 97, 110, 99, 111, 4, 97, 120, 101,
            108,
        ];
        let pkt = Nominees::from_bytes(bytes);
        assert_eq!(pkt.nominees, expected_packet.nominees);
    }
}
