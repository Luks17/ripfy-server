use std::time::Duration;

use anyhow::Result;
use argon2::password_hash::SaltString;
use ripfy_server::{
    crypt::{
        b64, decode_signature,
        passwd::{passwd_encrypt, verify_encrypted_passwd},
        sign_content,
        token::Token,
    },
    keys,
    util::time::now_utc_plus_sec_str,
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
    let decoded = b64::decode_to_string(b64_word.as_str())?;

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

#[test]
fn token() -> Result<()> {
    let access_token = Token::new_access_token("good guy")?;
    access_token.validate(&keys().verifying_key)?;

    let mut bad_token = Token::new_access_token("bad guy")?;
    bad_token.identifier = "really good guy".into();
    assert!(bad_token.validate(&keys().verifying_key).is_err());

    let name = "really late guy".to_string();
    let exp = now_utc_plus_sec_str(2)?;
    std::thread::sleep(Duration::from_secs(2));
    let expired_token = Token {
        identifier: name.clone(),
        expiration: exp.clone(),
        signature: sign_content(
            format!("{}.{}", b64::encode(name), b64::encode(exp)),
            &keys().signing_key,
        ),
    };
    assert!(expired_token.validate(&keys().verifying_key).is_err());

    Ok(())
}
