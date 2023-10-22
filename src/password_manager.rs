use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString,
    },
    Argon2
};
use base64::{Engine as _, engine::general_purpose};

pub fn password_hashing(_password: String) -> (String, SaltString) {

    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);

    let hashed_password = argon2.hash_password(b"{_password}", &salt).unwrap().to_string();
    let base_password = general_purpose::STANDARD_NO_PAD.encode(hashed_password);
    (base_password.to_string(), salt.clone())
}

pub fn password_verification_hashing(_old_password: String, old_salt: String, _new_password: String) -> bool {

    let argon2 = Argon2::default();
    let salted = SaltString::from_b64(&old_salt.as_str());
    let hashed_new_password = argon2.hash_password(b"{_new_password}", &salted).unwrap().to_string();
    let base_new_password = general_purpose::STANDARD_NO_PAD.encode(hashed_new_password);
    true
}