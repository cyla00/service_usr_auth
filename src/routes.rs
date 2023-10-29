use crate::structs::{RegistrationStruct, UserStruct, TokenClaimStruct, TokenStruct, ErrMsgStruct, SuccMsgStruct};
use crate::password_manager::{password_hashing, password_verification};
use crate::jwt_verification::jwt_verification;

use axum::headers::authorization::Bearer;
use axum::{
    http::StatusCode,
    Json, 
    extract::{State, TypedHeader},
    headers::{Authorization, authorization::Basic},
};

use common_regex_rs::*;
use mongodb::Collection;
use mongodb::{bson::doc, Database};
use uuid::Uuid;
use jsonwebtoken::{encode, Header, EncodingKey, get_current_timestamp};
use dotenv::dotenv;
use std::env;

pub async fn route_login(State(db): State<Database>, TypedHeader(auth): TypedHeader<Authorization<Basic>>) -> (StatusCode, Result<Json<TokenStruct>, Json<ErrMsgStruct>>) {
    
    let collection_name:&str = "users";

    let email = auth.username().to_string();

    let db_collection:Collection<UserStruct> = db.collection(collection_name);

    match db_collection.find_one(doc! {"email": &email}, None).await {
        Err(_) => {
            let err_msg = ErrMsgStruct {
                err_msg: "An error occurred, please retry later".to_string()
            };
            return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
        }
        Ok(None) => {
            let err_msg = ErrMsgStruct {
                err_msg: "Incorrect credentials".to_string()
            };
            return (StatusCode::UNAUTHORIZED, Err(Json(err_msg)))
        }
        Ok(Some(user)) => {
            let password = password_verification(user.password, auth.password().to_string());

            if !password {
                let err_msg = ErrMsgStruct {
                    err_msg: "Incorrect credentials".to_string()
                };
                return (StatusCode::UNAUTHORIZED, Err(Json(err_msg)))
            }
            else {

                let token_expiration = get_current_timestamp() + 10 * 1000;

                let claims: TokenClaimStruct = TokenClaimStruct {
                    id: user.id.to_string(),
                    exp: token_expiration,
                    iss: "ikwebdev".to_string(),
                };
    
                dotenv().ok();
                let sec_ = env::var("SEC_").unwrap();
                let token: TokenStruct = TokenStruct {
                    token: encode(&Header::default(), &claims, &EncodingKey::from_secret(&sec_.as_bytes())).unwrap(),
                    succ_msg: "Login successful".to_string(),
                };

                (StatusCode::CREATED, Ok(Json(token)))
            }
        } 
    }
}

pub async fn route_registration(State(db): State<Database>, Json(payload): Json<RegistrationStruct>) -> (StatusCode, Result<Json<SuccMsgStruct>, Json<ErrMsgStruct>>) {

    if payload.email.is_empty() | !is_email(&payload.email) {
        let err_msg: ErrMsgStruct = ErrMsgStruct {
            err_msg: "Provide a valid email".to_string()
        };
        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
    }

    if payload.password.is_empty() | !is_good_password(&payload.password) {
        let err_msg: ErrMsgStruct = ErrMsgStruct {
            err_msg: "Provide a valid and strong password".to_string()
        };
        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
    }

    let collection_name:&str = "users";
    let hashed_password = password_hashing(payload.password.to_string());
    let new_user_id:Uuid = Uuid::new_v4();
    let user: UserStruct = UserStruct {
        id: new_user_id.to_string(),
        email: payload.email,
        password: hashed_password.0,
        salt: hashed_password.1.to_string(),
        role: "user".to_string(),
    };
    let db_collection = db.collection(collection_name);
    match db_collection.find_one(doc! {"email": user.email.clone()}, None).await {
        Err(_) => {
            let err_msg: ErrMsgStruct = ErrMsgStruct {
                err_msg: "An error occurred, please retry later".to_string()
            };
            return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
        }
        Ok(Some(_)) => {
            let err_msg: ErrMsgStruct = ErrMsgStruct {
                err_msg: "You are already registered, connect to your account".to_string()
            };
            return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
        }
        Ok(None) => {
            match db_collection.insert_one(user, None).await {
                Err(_) => {
                    let err_msg: ErrMsgStruct = ErrMsgStruct {
                        err_msg: "An error occurred, please retry later".to_string()
                    };
                    return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
                }
                Ok(_) => {
                    let succ_msg: SuccMsgStruct = SuccMsgStruct {
                        succ_msg: "Successfully registered".to_string()
                    };
                    return (StatusCode::CREATED, Ok(Json(succ_msg)))
                }
            }
        }
    }
}

pub async fn test_jwt(TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> StatusCode {
    let token = auth.token();
    let verification = jwt_verification(token.to_string());
    match verification {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}