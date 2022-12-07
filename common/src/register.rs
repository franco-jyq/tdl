use crate::{
    packet_traits::{PasswordTobytes, ToBytes, ToBytesWithPassAndUser, UsernameToBytes},
    packet_type::PacketType,
};

static MAX_USERNAME_SIZE: u8 = 255;
static MAX_PASSWORD_SIZE: u8 = 255;
static MAX_EMAIL_SIZE: u8 = 255;

pub struct Register {
    packet_type: PacketType,
    username: String,
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
            packet_type: PacketType::Register,
            username,
            password,
            email_size,
            email,
        })
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    // En este si veo más lógica en un chequeo de error
    pub fn from_bytes(bytes: Vec<u8>) -> Register {
        let mut i = 0; // Contador

        let username_size = bytes[1] as usize;
        i += username_size + 2;
        let username = String::from_utf8(bytes[2..i].to_vec()).unwrap();

        let password_size = bytes[i] as usize;
        i += 1 + password_size;
        let password = String::from_utf8(bytes[username_size + 3..i].to_vec()).unwrap();

        let email_size = bytes[i] as usize;
        i += 1 + email_size;
        let email =
            String::from_utf8(bytes[password_size + username_size + 4..i].to_vec()).unwrap();

        Register::new(username, password, email).unwrap() // Chequeo?
    }
}

impl UsernameToBytes for Register {
    fn get_username(&self) -> &str {
        &self.username
    }

    fn get_packet_type(&self) -> PacketType {
        self.packet_type.clone()
    }
}

impl PasswordTobytes for Register {
    fn get_password(&self) -> &str {
        &self.password
    }
}

impl ToBytesWithPassAndUser for Register {
    fn to_bytes_with_pass(&self) -> Vec<u8> {
        let email_size_bytes = self.email_size.to_be_bytes().to_vec();
        let email_bytes = self.email.as_bytes().to_vec();
        [self.to_bytes_login_data(), email_size_bytes, email_bytes].concat()
    }
}

impl ToBytes for Register {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_with_pass()
    }
}

#[cfg(test)]

mod register_tests {

    use crate::packet_traits::ToBytes;

    use super::Register;

    #[test]
    fn register_to_bytes_test() {
        let test_packet = Register::new(
            "user".to_string(),
            "pass".to_string(),
            "user@pass".to_string(),
        )
        .unwrap();
        let expected = vec![
            1, 4, 117, 115, 101, 114, 4, 112, 97, 115, 115, 9, 117, 115, 101, 114, 64, 112, 97,
            115, 115,
        ];
        assert_eq!(test_packet.to_bytes(), expected);
    }

    #[test]
    fn register_from_bytes_test() {
        let bytes = vec![
            1, 4, 117, 115, 101, 114, 4, 112, 97, 115, 115, 9, 117, 115, 101, 114, 64, 112, 97,
            115, 115,
        ];
        let pkt = Register::from_bytes(bytes);
        assert_eq!(pkt.username, "user".to_string());
        assert_eq!(pkt.password, "pass".to_string());
        assert_eq!(pkt.email, "user@pass".to_string())
    }
}
