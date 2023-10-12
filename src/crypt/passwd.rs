use argon2::{
    password_hash::{Error, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn passwd_encrypt(passwd: impl Into<String>, salt: &str) -> Result<String, Error> {
    let salt_string = SaltString::from_b64(salt)?;

    let passwd_hash = Argon2::default().hash_password(passwd.into().as_bytes(), &salt_string)?;

    Ok(passwd_hash.to_string())
}

pub fn verify_encrypted_passwd(
    provided_passwd: impl Into<String>,
    hashed_passwd: &str,
) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hashed_passwd)?;
    let verify_passwd = Argon2::default()
        .verify_password(provided_passwd.into().as_bytes(), &parsed_hash)
        .is_ok();

    Ok(verify_passwd)
}
