use crate::PacketType;

static MAX_USERNAME_SIZE: u8 = 255;
static MAX_PASSWORD_SIZE: u8 = 255;
static MAX_EMAIL_SIZE: u8 = 255;

pub struct Register {
    packet_type: PacketType,
    username_size: u8,
    username: String,
    password_size: u8,
    password: String,
    email_size: u8,
    email: String,
}

impl Register {
    pub fn new(username: String, password: String, email: String) -> Result<Register, String> {
        let username_size = username.len() as u8;
        let password_size: u8 = password.len() as u8;
        let email_size = email.len() as u8;

        if username_size > MAX_USERNAME_SIZE {
            return Err(String::from("INVALID_USERNAME_SIZE"));
        }

        if password_size > MAX_PASSWORD_SIZE {
            return Err(String::from("INVALID_PASSWORD_SIZE"));
        }

        if email_size > MAX_EMAIL_SIZE {
            return Err(String::from("INVALID_EMAIL_SIZE"));
        }

        Ok(Register {
            packet_type: PacketType::REGISTER,
            username_size,
            username,
            password_size,
            password,
            email_size,
            email,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let packet_type_bytes = self.packet_type as u8;
        let username_size_bytes = self.username_size.to_be_bytes().to_vec();
        let username_bytes = self.username.as_bytes().to_vec();
        let password_size_bytes = self.password_size.to_be_bytes().to_vec();
        let password_bytes = self.password.as_bytes().to_vec();
        let email_size_bytes = self.email_size.to_be_bytes().to_vec();
        let email_bytes = self.email.as_bytes().to_vec();
        [
            packet_type_bytes.to_be_bytes().to_vec(),
            username_size_bytes,
            username_bytes,
            password_size_bytes,
            password_bytes,
            email_size_bytes,
            email_bytes,
        ]
        .concat()
    }
}
