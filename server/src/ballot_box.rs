use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::RwLock,
};

pub struct BallotBox {
    pub nominees: RwLock<HashMap<String, usize>>,
}

impl BallotBox {
    pub fn load_ballot(file_path: String) -> Result<BallotBox, String> {
        let file: File = match File::open(file_path) {
            Ok(data_file) => data_file,
            Err(_) => {
                return Err(String::from("Archivo para Ballot Box no encontrado"));
            }
        };
        let reader = BufReader::new(file);
        let mut nominees = HashMap::new();
        load_nominees(&mut nominees, reader);
        info!("Ballot Box inicializada correctamente");
        Ok(BallotBox {
            nominees: RwLock::new(nominees),
        })
    }

    pub fn vote_nominee(&self, nominee: &str, amount: usize) -> Result<(), String> {
        if let Ok(nominees) = &mut self.nominees.write() {
            if nominees.contains_key(nominee) {
                if let Some(votes) = nominees.get(nominee) {
                    let new_votes = votes + amount;
                    nominees.insert(nominee.to_string(), new_votes);
                }
            } else {
                return Err(String::from("NOT_A_NOMINEE"));
            }
        }
        update_data_base(&self.nominees);

        Ok(())
    }

    pub fn get_votes(&self) -> String {
        let mut nominee_votes: Vec<(String, usize)> = vec![];

        if let Ok(nominees) = self.nominees.read() {
            let nominees_clone = nominees.clone();
            for (nominee, _) in nominees_clone.into_iter() {
                if let Some(votes) = nominees.get(&nominee) {
                    nominee_votes.push((nominee, *votes));
                } else {
                    continue;
                }
            }
        }
        let mut votes = String::new();
        for v in nominee_votes {
            votes = votes + &v.0 + "," + &v.1.to_string() + ";";
        }
        votes
    }

    pub fn get_nominees(&self) -> Vec<String> {
        let mut nominee_vec = vec![];

        if let Ok(nominees) = self.nominees.read() {
            let nominees_clone = nominees.clone();
            for (nominee, _) in nominees_clone.into_iter() {
                nominee_vec.push(nominee)
            }
        }
        nominee_vec
    }
}
fn load_nominees(nominees: &mut HashMap<String, usize>, reader: BufReader<File>) {
    for line in reader.lines() {
        if let Ok(linea) = line {
            let vector_split = linea.split(',').collect::<Vec<&str>>();
            let nominee = vector_split[0].to_string();
            if let Ok(votes) = vector_split[1].to_string().parse() {
                nominees.entry(nominee).or_insert(votes);
            }
        } else {
            println!("Hubo un error con la linea del archivo para la base de datos",);
        }
    }
}

fn update_data_base(nominees: &RwLock<HashMap<String, usize>>) {
    if let Ok(mut file) = File::create("./src/ballot_data_base") {
        if let Ok(nominees) = nominees.read() {
            let nominees_clone = nominees.clone();
            for (nominee, vote) in nominees_clone.into_iter() {
                println!("Vote to be updated: {:?}", vote);
                let vector: Vec<String> = vec![nominee, vote.to_string()];
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
