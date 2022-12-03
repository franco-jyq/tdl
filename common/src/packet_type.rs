#[derive(Debug,Clone)]
pub enum PacketType {
    REGISTER,
    LOGIN,
    PAYMENT,
    DEFAULT,
    VOTE,
    ERROR,
    INFO,
    REQUEST,
    NOMINEES,
}

impl PacketType {
    pub fn from_utf8(value: u8) -> PacketType {
        match value {
            0 => PacketType::REGISTER,
            1 => PacketType::LOGIN,
            2 => PacketType::PAYMENT,
            3 => PacketType::VOTE,
            4 => PacketType::ERROR,
            5 => PacketType::INFO,
            6 => PacketType::REQUEST,
            7 => PacketType::NOMINEES,
            _ => PacketType::DEFAULT,
        }
    }

    pub fn as_utf8(&self) -> u8 {
        match self {
            PacketType::REGISTER => 0_u8,
            PacketType::LOGIN => 1_u8,
            PacketType::PAYMENT => 2_u8,
            PacketType::VOTE => 3_u8,
            PacketType::ERROR => 4_u8,
            PacketType::INFO => 5_u8,
            PacketType::REQUEST => 6_u8,
            PacketType::NOMINEES => 7_u8,
            _ => 10_u8,
        }
    }
}
