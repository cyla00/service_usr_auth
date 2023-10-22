use crate::structs::{RegistrationStruct, LoginStruct, UserStruct, TokenStruct};
use crate::password_manager::{password_hashing, password_verification_hashing};

use axum::{
    http::StatusCode,
    Json,
};
#[allow(unused_imports)]
use mongodb::{Client, options::ClientOptions, error::Result, bson::doc, Database};
use uuid::Uuid;

pub async fn route_login(_payload: Json<LoginStruct>) -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "Login Successful")
}


pub async fn route_registration(Json(payload): Json<RegistrationStruct>) -> (StatusCode, &'static str) {

    let db = crate::mongo::database_connection();
    let collection_name:&str = "users";

    let hashed_password = password_hashing(payload.password);

    let new_user_id:Uuid = Uuid::new_v4();

    let user: UserStruct = UserStruct {
        id: new_user_id.to_string(),
        email: payload.email,
        password: hashed_password.0,
        salt: hashed_password.1.to_string(),
    };

    let found_user: Option<UserStruct> = db.await.collection(collection_name).find_one(doc! {"email": user.email.clone()}, None).await.unwrap();

    if found_user.unwrap().email == user.email.clone() {
        return (StatusCode::NOT_FOUND, "You are already registered, connect to your account");
    };

    let db = crate::mongo::database_connection();
    db.await.collection(collection_name).insert_one(user.clone(), None).await.unwrap();

    (StatusCode::CREATED, "Registration Successful")
}
