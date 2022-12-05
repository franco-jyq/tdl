use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::RwLock,
};

use crate::user::User;




pub struct DataBase {
    clients: RwLock<HashMap<String, User>>,
}

impl DataBase {
    pub fn new(file_path: &str) -> Result<DataBase, String> {
        let file: File = match File::open(file_path) {
            Ok(data_file) => data_file,
            Err(_) => {
                return Err(String::from("Archivo para Data Base no encontrado"));
            }
        };
        let reader = BufReader::new(file);
        let mut clients = HashMap::new();
        load_users(&mut clients, reader);
        info!("Data Base inicializada correctamente");
        Ok(DataBase {
            clients: RwLock::new(clients),
        })
    }

    pub fn log_new_user(&self, username: String, password: String) -> Result<(), String> {
        if let Ok(clients) = &mut self.clients.read() {
            if let Some(user) = clients.get(&username) {
                if user.password == password {
                    return Ok(());
                }
                return Err(String::from("INVALID_PASSWORD"));
            }
            return Err(String::from("USERNAME_NOT_FOUND"));
        }
        Err(String::from("SERVER_FATAL_ERROR"))
    }

    pub fn save_new_user(
        &self,
        username: String,
        password: String,
        email: String,
    ) -> Result<(), String> {
        if let Ok(clients) = &mut self.clients.write() {
            if clients.contains_key(&username) {
                return Err(String::from("USERNAME_IN_USE"));
            }
            clients.insert(
                username.to_string(),
                User::new(vec![&username, &password, &email, "0"]),
            );

            println!("DataBase:{:?}", clients);
        }

        update_data_base(&self.clients);
        Ok(())
    }

    pub fn get_money(&self,username:&String) -> Result<u32, String>{
        if let Ok(clients) = &mut self.clients.write() {
            let user = clients.get_mut(username).unwrap();
            return Ok(user.balance);
        }
        Err(String::from("GET MONEY ERR"))
    }

    pub fn update_money(&self, username: String, amount: u32) -> Result<u32, String> {
        let mut updated_balance = amount;
        if let Ok(clients) = &mut self.clients.write() {
            match clients.get_mut(&username) {
                Some(user) => {
                    user.balance += amount;
                    updated_balance = user.balance;
                }
                None => return Err(String::from("USER NOT LOGGED")),
            }
        }
        update_data_base(&self.clients);
        Ok(updated_balance)
    }

    pub fn can_vote(&self, username: &str, amount: u32) -> Result<u32, String> {
        if let Ok(clients) = &mut self.clients.write() {
            let user = clients.get_mut(username).unwrap();
            println!("{}{}", user.balance, amount);
            if user.balance >= amount {
                return Ok(user.balance);
            } else {
                return Err(format!("INSUFFICIENT FUNDS: {}", user.balance));
            }
        }

        Err(String::from("SERVER FATAL ERROR"))
    }

    pub fn update_user_balance(
        &self,
        amount_to_decrease: u32,
        username: &str,
    ) -> Result<u32, String> {
        if let Ok(clients) = &mut self.clients.write() {
            let user = clients.get_mut(username).unwrap();
            user.balance -= amount_to_decrease;
        }
        update_data_base(&self.clients);
        Ok(2)
    }
}

fn load_users(clients: &mut HashMap<String, User>, reader: BufReader<File>) {
    for line in reader.lines() {
        if let Ok(linea) = line {
            let vector_split = linea.split(',').collect::<Vec<&str>>();
            let username = vector_split[0].to_string();
            let user = User::new(vector_split); // tiene que ser mutable para despues
            clients.entry(username).or_insert(user);
        } else {
            println!("Hubo un error con la linea del archivo para la base de datos",);
        }
    }
}

fn update_data_base(clients: &RwLock<HashMap<String, User>>) {
    if let Ok(mut file) = File::create("./src/data_file") {
        if let Ok(clients) = clients.read() {
            let clients_clone = clients.clone();
            for (_client, user) in clients_clone.into_iter() {
                let vector: Vec<String> = vec![
                    user.username,
                    user.password,
                    user.email,
                    user.balance.to_string(),
                ];
                let comma: String = String::from(",");
                for atribute in vector.iter() {
                    file.write_all(atribute.as_bytes())
                        .expect("Unable to write data");
                    file.write_all(comma.as_bytes())
                        .expect("Unable to write data");
                }
                let jump = String::from("\n");
                file.write_all(jump.as_bytes())
                    .expect("Unable to write data");
            }
        }
    }
}
