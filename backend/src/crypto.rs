use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub(crate) fn hash(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_ref(), &salt)?.to_string()
}

pub(crate) fn verify(password: String, password_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(password_hash)?;
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_ref(), &parsed_hash).is_ok()
}
