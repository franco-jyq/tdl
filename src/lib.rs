pub mod cliente;
pub mod login;
pub mod register;
pub mod threadpool;

pub enum PacketType {
    REGISTER,
    LOGIN,
}
