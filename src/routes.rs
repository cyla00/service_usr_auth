use crate::structs::{RegistrationStruct, LoginStruct, UserStruct, TokenStruct};
use crate::password_manager::{password_hashing, password_hash_verification};

use axum::{
    http::StatusCode,
    Json, extract::State,
};
use common_regex_rs::*;
#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions, error::Result, bson::doc, Database};
use uuid::Uuid;

pub async fn route_login(State(_db): State<Database>, _payload: Json<LoginStruct>) -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Login Successful")
}

pub async fn route_registration(State(db): State<Database>, Json(payload): Json<RegistrationStruct>) -> (StatusCode, &'static str) {

    if payload.email.is_empty() | !is_email(&payload.email) {
        return (StatusCode::BAD_REQUEST, "Provide a valid email")
    }

    if payload.password.is_empty() | !is_good_password(&payload.password) {
        return (StatusCode::BAD_REQUEST, "Provide a valid and strong password")
    }

    let collection_name:&str = "users";
    let hashed_password = password_hashing(payload.password.to_string());
    let new_user_id:Uuid = Uuid::new_v4();
    let user: UserStruct = UserStruct {
        id: new_user_id.to_string(),
        email: payload.email,
        password: hashed_password.0,
        salt: hashed_password.1.to_string(),
    };
    let db_collection = db.collection(collection_name);
    match db_collection.find_one(doc! {"email": &user.email}, None).await {
        Err(_) => return (StatusCode::BAD_GATEWAY, "An error occurred, please retry later"),
        Ok(Some(_)) => return (StatusCode::BAD_REQUEST, "You are already registered, connect to your account"),
        Ok(None) => {
            match db_collection.insert_one(user, None).await {
                Err(_) => return (StatusCode::BAD_GATEWAY, "An error occurred, please retry later"),
                Ok(_) => return (StatusCode::CREATED, "Successfully registered")
            }
        }
    }
}
