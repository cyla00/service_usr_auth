use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct RegistrationStruct{
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserStruct{
    pub id: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenStruct {
    pub token: String,
    pub succ_msg: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaimStruct {
    pub id: String,
    pub exp: usize,
    pub iss: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrMsgStruct {
    pub err_msg: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SuccMsgStruct {
    pub succ_msg: String,
}