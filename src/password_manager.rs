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

    println!("original salt: {}", salt);

    let hashed_password = argon2.hash_password(b"{_password}", &salt).unwrap().to_string();
    let base_password = general_purpose::STANDARD_NO_PAD.encode(hashed_password);

    println!("original pass: {}", base_password);

    (base_password.to_string(), salt.clone())
}

pub fn password_hash_from_salt(password: String, salt: String) -> String {
    let argon2 = Argon2::default();
    let salted = SaltString::from_b64(&salt.as_ref()).unwrap();

    println!("new salt {}", salted);

    let hashed_password = argon2.hash_password(b"{password}", &salted).unwrap().to_string();
    return general_purpose::STANDARD_NO_PAD.encode(hashed_password);
}