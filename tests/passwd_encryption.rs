use anyhow::Result;
use argon2::password_hash::{rand_core::OsRng, SaltString};
use ripfy_server::crypt::{passwd_encrypt, verify_encrypted_passwd};

#[test]
fn encrypted_passwd() -> Result<()> {
    let passwd = "abcde123";
    let salt = SaltString::generate(&mut OsRng).to_string();

    let encrypted_passwd = passwd_encrypt(passwd, &salt)?;
    let do_passwords_match = verify_encrypted_passwd(passwd, &encrypted_passwd)?;
    let do_passwords_not_match = !verify_encrypted_passwd("ABCDE123", &encrypted_passwd)?;

    assert!(do_passwords_match);
    assert!(do_passwords_not_match);

    Ok(())
}