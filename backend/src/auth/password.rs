use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier, SaltString, PasswordHash},
    Argon2,
};
use rand::rngs::OsRng;

pub async fn hash() {
    // placeholder
}

pub async fn verify_password() {
    // placeholder
}

pub fn password_hash(password: String) -> String {
    let mut rng = OsRng;

    let salt = SaltString::generate(&mut rng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("Unable to hash password.")
        .to_string()
}

pub fn verify(password: String, hash: String) -> bool {
    let parsed_hash = PasswordHash::new(&hash)
        .expect("Failed to parse hash");

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}