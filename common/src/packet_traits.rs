use crate::packet_type::PacketType;

pub trait UsernameToBytes {
    fn get_username (&self) -> &str;
    fn get_packet_type(&self) -> PacketType;

    fn pkt_type_and_username_to_bytes (&self) -> Vec<u8> {
        let packet_type_bytes = self.get_packet_type().as_utf8().to_be_bytes().to_vec();
        let username = self.get_username();
        let username_size = username.len() as u8;
        let username_size_bytes = username_size.to_be_bytes().to_vec();
        let username_bytes= username.as_bytes().to_vec();
        [packet_type_bytes,username_size_bytes,
            username_bytes].concat()      
    }
}

pub trait GetPassword {
    fn get_password(&self) -> &str;
}

pub trait ToBytesWithPass : UsernameToBytes + GetPassword {   
    
    fn password_to_bytes (&self) -> Vec<u8> {
        let password = self.get_password();
        let password_size: u8 = password.len() as u8;
        let password_size_bytes = password_size.to_be_bytes().to_vec();
        let password_bytes= password.as_bytes().to_vec();
        [password_size_bytes,
            password_bytes].concat()      
    }
    
    fn to_bytes_login_data(&self) -> Vec<u8> {
        [self.pkt_type_and_username_to_bytes(),
        self.password_to_bytes()].concat()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_login_data()   
    }
}