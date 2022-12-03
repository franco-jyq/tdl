use crate::packet_type::PacketType;

pub struct InfoPacket {
    packet_type: PacketType,
    info_msg_size: u8,
    info_msg: String,
}

impl InfoPacket {
    pub fn new(pkt_type: PacketType, info_msg: String) -> InfoPacket {
        let info_msg_size = info_msg.len() as u8;
        InfoPacket {
            packet_type: pkt_type,
            info_msg_size,
            info_msg,
        }
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let packet_type_bytes = self.packet_type.as_utf8().to_be_bytes().to_vec();
        let info_msg_size_bytes = self.info_msg_size.to_be_bytes().to_vec();
        let info_msg_bytes = self.info_msg.as_bytes().to_vec();
        [packet_type_bytes, info_msg_size_bytes, info_msg_bytes].concat()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> InfoPacket {
        let mut i = 0;
        let pkt_type = PacketType::from_utf8(bytes[0]);
        let info_msg_size = bytes[1] as usize;
        i += info_msg_size + 2;
        let info_msg = String::from_utf8(bytes[2..i].to_vec()).unwrap();
        InfoPacket::new(pkt_type, info_msg)
    }

    pub fn is_err(&mut self) -> bool{
        self.packet_type.as_utf8() == PacketType::ERROR.as_utf8()
    }

    pub fn get_msg(&mut self) -> String{
        self.info_msg.clone()
    }

}

#[cfg(test)]

mod info_packet_test {
    use crate::packet_type::PacketType;

    use super::InfoPacket;

    #[test]
    fn info_packet_to_bytes_test() {
        let mut test_packet = InfoPacket::new(PacketType::ERROR, "error".to_string());

        let expected = vec![4, 5, 101, 114, 114, 111, 114];
        assert_eq!(test_packet.to_bytes(), expected);
    }

    #[test]
    fn info_packet_from_bytes_test() {
        let bytes = vec![4, 10, 102, 97, 116, 97, 108, 101, 114, 114, 111, 114];

        let pkt = InfoPacket::from_bytes(bytes);
        assert_eq!(pkt.info_msg, "fatalerror".to_string());
    }
}
