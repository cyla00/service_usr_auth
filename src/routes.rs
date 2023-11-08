use crate::structs::{RegistrationStruct, UserStruct, TokenClaimStruct, TokenStruct, ErrMsgStruct, SuccMsgStruct, UserEmailStruct};
use crate::password_manager::{password_hashing, password_verification};
use crate::jwt_verification::jwt_verification;
use crate::mailer::{user_verification_email, password_recovery_email};

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
            else if !user.active {
                let err_msg = ErrMsgStruct {
                    err_msg: "Verify your account before connecting".to_string()
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

    if &payload.email.is_empty() | !is_email(&payload.email) {
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
    let new_user_hash:Uuid = Uuid::new_v4();
    let user: UserStruct = UserStruct {
        id: new_user_id.to_string(),
        hash: new_user_hash.to_string(),
        email: payload.email.clone(),
        password: hashed_password.0,
        salt: hashed_password.1.to_string(),
        role: "user".to_string(),
        active: false,
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
                    let platform_name = env::var("PLATFORM_NAME").unwrap();
                    let platform_host = env::var("PLATFORM_HOST").unwrap();
                    let email_auth_host = env::var("EMAIL_CLIENT_HOST").unwrap();
                    let auth_email_user = env::var("EMAIL_CLIENT_USER").unwrap();
                    let email_auth_pass = env::var("EMAIL_CLIENT_PASS").unwrap();
                    let mailer = user_verification_email(
                        auth_email_user, 
                        email_auth_pass, 
                        email_auth_host, 
                        payload.email, 
                        platform_name, 
                        new_user_hash.to_string(), 
                        platform_host
                    ).await;
    
                    match mailer {
                        Ok(_) => {
                            let succ_msg: SuccMsgStruct = SuccMsgStruct {
                                succ_msg: "Successfully registered".to_string()
                            };
                            return (StatusCode::CREATED, Ok(Json(succ_msg)))
                        },
                        Err(_) => {
                            let err_msg: ErrMsgStruct = ErrMsgStruct {
                                err_msg: "An error occurred, please retry later".to_string()
                            };
                            return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
                        }
                    }
                }
            }
        }
    }
}

pub async fn jwt_auth(TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> StatusCode {

    let token = auth.token();
    let verification = jwt_verification(token.to_string());
    match verification {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}

pub async fn verify_account(State(db): State<Database>, TypedHeader(auth): TypedHeader<Authorization<Bearer>>) -> (StatusCode, Result<Json<SuccMsgStruct>, Json<ErrMsgStruct>>) {
    let collection_name:&str = "users";
    let hash = auth.token().to_string();
    let db_collection:Collection<UserStruct> = db.collection(collection_name);
    match db_collection.find_one(doc! {"hash": &hash}, None).await {
        Err(_) => {
            let err_msg = ErrMsgStruct {
                err_msg: "An error occurred, retry later".to_string()
            };
            return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
        }
        Ok(None) => {
            let err_msg = ErrMsgStruct {
                err_msg: "An error occurred, retry later".to_string()
            };
            return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
        }
        Ok(Some(_)) => {
            let new_hash = Uuid::new_v4();
            match db_collection.update_one(doc! {"hash": &hash}, doc! {"$set": {"hash": new_hash.to_string(), "active": true}}, None).await {
                Err(_) => {
                    let err_msg = ErrMsgStruct {
                        err_msg: "An error occurred, retry later".to_string()
                    };
                    return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
                }
                Ok(_) => {
                    let succ_msg = SuccMsgStruct {
                        succ_msg: "Account verified".to_string()
                    };
                    return (StatusCode::OK, Ok(Json(succ_msg)))
                }
            }
        }
    }
}

pub async fn request_password_recovery_email(State(db): State<Database>, Json(payload): Json<UserEmailStruct>) -> (StatusCode, Result<Json<SuccMsgStruct>, Json<ErrMsgStruct>>){

    let collection_name:&str = "users";
    let db_collection:Collection<UserStruct> = db.collection(collection_name);

    if &payload.email.is_empty() | !is_email(&payload.email) {
        let err_msg: ErrMsgStruct = ErrMsgStruct {
            err_msg: "Provide a valid email".to_string()
        };
        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
    }

    match db_collection.find_one(doc! {"email": &payload.email}, None).await {
        Err(_) => {
            let err_msg = ErrMsgStruct {
                err_msg: "An error occurred, retry later".to_string()
            };
            (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
        }
        Ok(None) => {
            let err_msg = ErrMsgStruct {
                err_msg: "Incorrect credentials".to_string()
            };
            (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
        }
        Ok(Some(user)) => {

            let platform_name = env::var("PLATFORM_NAME").unwrap();
            let platform_host = env::var("PLATFORM_HOST").unwrap();
            let email_auth_host = env::var("EMAIL_CLIENT_HOST").unwrap();
            let auth_email_user = env::var("EMAIL_CLIENT_USER").unwrap();
            let email_auth_pass = env::var("EMAIL_CLIENT_PASS").unwrap();

            if !user.active {
                let err_msg: ErrMsgStruct = ErrMsgStruct {
                    err_msg: "Activate your account before proceeding".to_string()
                };
                return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
            }

            let mailer = password_recovery_email(
                email_auth_host, 
                auth_email_user, 
                email_auth_pass, 
                platform_host, 
                payload.email, 
                platform_name, 
                user.hash
            ).await;

            match mailer {
                Ok(_) => {
                    let succ_msg: SuccMsgStruct = SuccMsgStruct {
                        succ_msg: "Recovery email sent".to_string()
                    };
                    (StatusCode::OK, Ok(Json(succ_msg)))
                },
                Err(_) => {
                    let err_msg: ErrMsgStruct = ErrMsgStruct {
                        err_msg: "An error occurred, please retry later".to_string()
                    };
                    (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
                }
            }
        }
    }
}