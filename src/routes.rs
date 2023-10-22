use crate::structs::{RegistrationStruct, UserStruct};
use crate::password_manager::password_hashing;

use axum::{
    http::StatusCode,
    Json,
};
use uuid::Uuid;

pub async fn route_login() -> &'static str {
    "login route"
}

pub async fn route_registration(Json(payload): Json<RegistrationStruct>) -> (StatusCode, Json<UserStruct>) {


    let collection_name:&str = "users";

    let hashed_password = password_hashing(payload.password);

    let new_user_id:Uuid = Uuid::new_v4();

    let user: UserStruct = UserStruct {
        id: new_user_id.to_string(),
        email: payload.email,
        password: hashed_password.0,
        salt: hashed_password.1.to_string(),
    };

    let db = crate::mongo::database_connection();
    db.await.collection(collection_name).insert_one(user.clone(), None).await.unwrap();
    println!("user {} added to database", user.id.clone());

    (StatusCode::CREATED, Json(user.clone()))
}
