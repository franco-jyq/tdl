use crate::{packet_type::PacketType, packet_traits::{UsernameToBytes, GetPassword, ToBytesWithPass}};

static MAX_USERNAME_SIZE: u8 = 255;
static MAX_PASSWORD_SIZE: u8 = 255;

pub struct Login {
    packet_type: PacketType,
    pub username: String,
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
            username,
            password,
        })
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

impl UsernameToBytes for Login {
    fn get_username (&self) -> &str {
        &self.username
    }

    fn get_packet_type(&self) -> PacketType {
        self.packet_type.clone()
    }
}

impl GetPassword for Login {
    fn get_password(&self) -> &str{
        &self.password
    }
}

impl ToBytesWithPass for Login { }

#[cfg(test)]

mod login_tests {

    use crate::login::ToBytesWithPass;
    use super::Login;

    #[test]
    fn login_to_bytes_test() {
        let test_packet = Login::new(
            "user".to_string(),
            "pass".to_string(),
        )
        .unwrap();
        let expected = vec![
            1, 4, 117, 115, 101, 114, 4, 112, 97, 115,115
        ];
        assert_eq!(test_packet.to_bytes(), expected);
    }

    #[test]
    fn login_from_bytes_test() {
        let bytes = vec![
            0, 4, 117, 115, 101, 114, 4, 112, 97, 115, 115, 9, 117, 115, 101, 114, 64, 112, 97,
            115, 115,
        ];
        let pkt = Login::from_bytes(bytes);
        assert_eq!(pkt.username, "user".to_string());
        assert_eq!(pkt.password, "pass".to_string())
    }
}
