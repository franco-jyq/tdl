static MAX_ARG_VOTE: usize = 1;

#[derive(Debug, Clone)]
pub struct PayInfo {
    pub amount: String,
}

impl PayInfo {
    pub fn new(parametro: Vec<String>) -> Result<PayInfo, String> {
        if parametro.len() < MAX_ARG_VOTE {
            return Err(String::from("ERR_NEEDMOREPARAMS"));
        }
        if parametro.len() > MAX_ARG_VOTE + 1 {
            return Err(String::from("ERR_MOREPARAMS"));
        }
        Ok(PayInfo {
            amount: parametro[0].to_string(),

        })
    }
}