use serde::{Deserialize, Serialize};


// STATE STRUCTS



// CONFIG STRUCTS
#[derive(Deserialize)]
pub struct Data {
    pub config: Config,
    pub test: Test,
}
#[derive(Deserialize)]
pub struct Config {
    pub enable_smtp: bool,
}
#[derive(Deserialize)]
pub struct Test {
    pub test_content: String,
}


// USER DATA STRUCTS
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenStruct {
    pub token: String,
    pub succ_msg: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaimStruct {
    pub id: String,
    pub exp: u64,
    pub iss: String,
}


// SERVER MESSAGES STRUCTS
#[derive(Serialize, Deserialize, Clone)]
pub struct ErrMsgStruct {
    pub err_msg: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SuccMsgStruct {
    pub succ_msg: String,
}