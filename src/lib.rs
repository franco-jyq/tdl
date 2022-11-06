pub mod cliente;
pub mod connection;
pub mod register;
pub mod threadpool;

#[derive(Debug)]
pub enum PacketType {
    REGISTER,
    LOGIN,
    DEFAULT,
}

impl PacketType {
    pub fn from_utf8(value: u8) -> PacketType {
        match value {
            0 => PacketType::REGISTER,
            1 => PacketType::LOGIN,
            _ => PacketType::DEFAULT,
        }
    }
}
