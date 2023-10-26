use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, SaltString, PasswordVerifier,
    },
    Argon2
};
use base64::{Engine as _, engine::general_purpose};


pub fn password_hashing(password: String) -> (String, SaltString) {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default().hash_password(&password.as_bytes(), &salt).unwrap().to_string();
    let encoded_pass: String = general_purpose::STANDARD_NO_PAD.encode(hashed_password);
    (encoded_pass.to_string(), salt.clone())
}

pub fn password_verification(old_password: String, new_password: String) -> bool {

    let decoded_old_pass = general_purpose::STANDARD_NO_PAD.decode(&old_password).unwrap();
    let old_str_pass = String::from_utf8(decoded_old_pass).unwrap();
    let old_pass_hash = PasswordHash::new(&old_str_pass).unwrap();
    let pass_verification = Argon2::default().verify_password(&new_password.as_bytes(), &old_pass_hash);

    match pass_verification {
        Ok(_) => return true,
        Err(_) => return false,
    }
}