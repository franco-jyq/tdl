#[derive(Debug, Clone)]
pub enum PacketType {
    Register,
    Login,
    Payment,
    Vote,
    Error,
    Info,
    RequestNominees,
    RequestBalance,
    RequestResults,
    Nominees,   
    Default
}

impl PacketType {
    pub fn from_utf8(value: u8) -> PacketType {
        match value {
            1 => PacketType::Register,
            2 => PacketType::Login,
            3 => PacketType::Payment,
            4 => PacketType::Vote,
            5 => PacketType::Error,
            6 => PacketType::Info,
            7 => PacketType::RequestNominees,
            8 => PacketType::RequestBalance,
            9 => PacketType::RequestResults,
            10 => PacketType::Nominees,
            _ => PacketType::Default
        }
    }

    pub fn as_utf8(&self) -> u8 {
        match self {
            PacketType::Register => 1_u8,
            PacketType::Login => 2_u8,
            PacketType::Payment => 3_u8,
            PacketType::Vote => 4_u8,
            PacketType::Error => 5_u8,
            PacketType::Info => 6_u8,
            PacketType::RequestNominees => 7_u8,
            PacketType::RequestBalance => 8_u8,
            PacketType::RequestResults => 9_u8,
            PacketType::Nominees => 10_u8,
            _ => 11_u8
        }
    }
}
