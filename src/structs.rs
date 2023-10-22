use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegistrationStruct{
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Clone)]
pub struct UserStruct{
    pub id: String,
    pub email: String,
    pub password: String,
    pub salt: String,
}