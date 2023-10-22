use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct RegistrationStruct{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct LoginStruct {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserStruct{
    pub id: String,
    pub email: String,
    pub password: String,
    pub salt: String,
}

#[derive(Serialize, Clone)]
pub struct TokenStruct {
    pub token: String,
}