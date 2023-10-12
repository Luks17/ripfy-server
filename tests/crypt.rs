use anyhow::Result;
use argon2::password_hash::SaltString;
use ripfy_server::{
    crypt::{
        b64, decode_signature,
        passwd::{passwd_encrypt, verify_encrypted_passwd},
        sign_content,
    },
    keys,
};
use rsa::signature::Verifier;

#[test]
fn encrypted_passwd() -> Result<()> {
    let passwd = "abcde123";
    let mut rng = rand::thread_rng();
    let salt = SaltString::generate(&mut rng).to_string();

    let encrypted_passwd = passwd_encrypt(passwd, &salt)?;
    let do_passwords_match = verify_encrypted_passwd(passwd, &encrypted_passwd)?;
    let do_passwords_not_match = !verify_encrypted_passwd("ABCDE123", &encrypted_passwd)?;

    assert!(do_passwords_match);
    assert!(do_passwords_not_match);

    Ok(())
}

#[test]
fn simple_b64() -> Result<()> {
    let word = "abcde".to_string();

    let b64_word = b64::encode("abcde");
    let decoded = b64::decode(b64_word.as_str())?;

    assert_eq!(word, decoded);

    Ok(())
}

#[test]
fn signature() -> Result<()> {
    let content = "Really cool message";
    let content_alt = "Reall cool message";

    let signature_string = sign_content(content.into(), &keys().signing_key);
    let signature = decode_signature(signature_string.as_str())?;

    assert!(keys()
        .verifying_key
        .verify(content.as_bytes(), &signature)
        .is_ok());

    assert!(keys()
        .verifying_key
        .verify(content_alt.as_bytes(), &signature)
        .is_err());

    Ok(())
}
