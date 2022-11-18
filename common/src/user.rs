static USERNAME: usize = 0;
static PASSWORD: usize = 1;
static EMAIL: usize = 2;

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn new(vector: Vec<&str>) -> User {
    
        User {
            username: vector[USERNAME].to_string(),
            password: vector[PASSWORD].to_string(),
            email: vector[EMAIL].to_string(),
        }
    }

}