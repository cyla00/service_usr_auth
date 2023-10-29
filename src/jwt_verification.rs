use dotenv::dotenv;
use std::env;
use crate::structs::{TokenClaimStruct};
use jsonwebtoken::{Validation, DecodingKey, decode};

pub fn jwt_verification(token: String) -> Result<bool, bool> {
    dotenv().ok();

    let sec_ = env::var("SEC_").unwrap();
    let decoding = decode::<TokenClaimStruct>(&token, &DecodingKey::from_secret(&sec_.as_bytes()), &Validation::default());

    match decoding {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}