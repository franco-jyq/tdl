use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
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
                return Err(String::from("DATABASE_FILE_NOT_FOUND"));
            }
        };
        let reader = BufReader::new(file);
        let mut clients = HashMap::new();
        load_users(&mut clients, reader);

        Ok(DataBase {
            clients: RwLock::new(clients),
        })
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
                User::new(vec![&username, &password, &email], 0),
            );

            println!("DataBase:{:?}", clients);
        }

        //update_data_base();
        Ok(())
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

        Ok(updated_balance)
    }
}

/// Funcion que actualiza el archivo con el nuevo usuario
// fn update_data_base(&self){

// }

fn load_users(clients: &mut HashMap<String, User>, reader: BufReader<File>) {
    for line in reader.lines() {
        if let Ok(linea) = line {
            let vector_split = linea.split(',').collect::<Vec<&str>>();
            let username = vector_split[0].to_string();
            let user = User::new(vector_split, 0); // tiene que ser mutable para despues
            clients.entry(username).or_insert(user);
        } else {
            println!("Hubo un error con la linea del archivo para la base de datos",);
        }
    }
}