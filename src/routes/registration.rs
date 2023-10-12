use axum::{
    http::StatusCode,
    Json,
};

#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegistrationStruct{
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct UserStruct{
    id: Uuid,
    email: String,
    password: String,
}

pub async fn route_registration(Json(payload): Json<RegistrationStruct>) -> (StatusCode, Json<UserStruct>) {

    let new_user_id:Uuid = Uuid::new_v4();
    let user:UserStruct = UserStruct {
        id: new_user_id,
        email: payload.email,
        password: payload.password,
    };

    (StatusCode::CREATED, Json(user))
}