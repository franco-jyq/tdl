use crate::packet_type::PacketType;

static MAX_USERNAME_SIZE: u8 = 255;
static MAX_PASSWORD_SIZE: u8 = 255;

pub struct Login {
    packet_type: PacketType,
    username_size: u8,
    pub username: String,
    password_size: u8,
    pub password: String,
}

impl Login {
    pub fn new(username: String, password: String) -> Result<Login, String> {
        let username_size = username.len() as u8;
        let password_size: u8 = password.len() as u8;

        if username_size > MAX_USERNAME_SIZE {
            return Err(String::from("INVALID_USERNAME_SIZE"));
        }

        if password_size > MAX_PASSWORD_SIZE {
            return Err(String::from("INVALID_PASSWORD_SIZE"));
        }

        Ok(Login {
            packet_type: PacketType::LOGIN,
            username_size,
            username,
            password_size,
            password,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let packet_type_bytes = self.packet_type.as_utf8().to_be_bytes().to_vec();
        let username_size_bytes = self.username_size.to_be_bytes().to_vec();
        let username_bytes = self.username.as_bytes().to_vec();
        let password_size_bytes = self.password_size.to_be_bytes().to_vec();
        let password_bytes = self.password.as_bytes().to_vec();
        [
            packet_type_bytes,
            username_size_bytes,
            username_bytes,
            password_size_bytes,
            password_bytes,
        ]
        .concat()
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Login {
        let mut i = 0;

        let username_size = bytes[1] as usize;
        i += username_size + 2;
        let username = String::from_utf8(bytes[2..i].to_vec()).unwrap();

        let password_size = bytes[i] as usize;
        i += 1 + password_size;
        let password = String::from_utf8(bytes[username_size + 3..i].to_vec()).unwrap();

        Login::new(username, password).unwrap()
    }
}
/* 
#[cfg(test)]

mod login_tests {

    use super::Login;

    #[test]
    fn login_to_bytes_test() {
        let test_packet = Login::new(
            "user".to_string(),
            "pass".to_string(),
        )
        .unwrap();
        let expected = vec![
            0, 4, 117, 115, 101, 114, 4, 112, 97, 115, 115, 9, 117, 115, 101, 114, 64, 112, 97,
            115, 115,
        ];
        assert_eq!(test_packet.to_bytes(), expected);
    }

    #[test]
    fn register_from_bytes_test() {
        let bytes = vec![
            0, 4, 117, 115, 101, 114, 4, 112, 97, 115, 115, 9, 117, 115, 101, 114, 64, 112, 97,
            115, 115,
        ];
        let pkt = Register::from_bytes(bytes);
        assert_eq!(pkt.username, "user".to_string());
        assert_eq!(pkt.password, "pass".to_string());
        assert_eq!(pkt.email, "user@pass".to_string())
    }
}
*/