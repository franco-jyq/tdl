static MAX_ARG_VOTE: usize = 2;

#[derive(Debug, Clone)]
pub struct VoteInfo {
    pub nominated: String,
    pub vote_amount: String,
}

impl VoteInfo {
    pub fn new(parametro: Vec<String>) -> Result<VoteInfo, String> {
        if parametro.len() < MAX_ARG_VOTE {
            return Err(String::from("ERR_NEEDMOREPARAMS"));
        }
        if parametro.len() > MAX_ARG_VOTE + 1 {
            return Err(String::from("ERR_MOREPARAMS"));
        }
        Ok(VoteInfo {
            nominated: parametro[0].to_string(),
            vote_amount: parametro[1].to_string(),
        })
    }
}
