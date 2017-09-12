use std::error::Error;

extern crate sshkeys;

#[test]
fn test_rsa_pubkey_1024() {
    let key = sshkeys::PublicKey::from_path("tests/test-keys/id_rsa_1024.pub").unwrap();

    assert_eq!(key.key_type.name, "ssh-rsa");
    assert_eq!(key.key_type.short_name, "RSA");
    assert_eq!(key.key_type.is_cert, false);
    assert_eq!(key.key_type.kind, sshkeys::KeyTypeKind::KeyRsa);

    assert_eq!(key.bits(), 1024);
    assert_eq!(key.comment, None);

    let rsa = match key.kind {
        sshkeys::PublicKeyKind::Rsa(k) => k,
        _ => panic!("Expected an RSA public key"),
    };

    // TODO: Test the fingerprint
}

#[test]
fn test_rsa_pubkey_2048() {
    let key = sshkeys::PublicKey::from_path("tests/test-keys/id_rsa_2048.pub").unwrap();

    assert_eq!(key.key_type.name, "ssh-rsa");
    assert_eq!(key.key_type.short_name, "RSA");
    assert_eq!(key.key_type.is_cert, false);
    assert_eq!(key.key_type.kind, sshkeys::KeyTypeKind::KeyRsa);

    assert_eq!(key.bits(), 2048);
    assert_eq!(key.comment, Some(String::from("me@home")));

    let rsa = match key.kind {
        sshkeys::PublicKeyKind::Rsa(k) => k,
        _ => panic!("Expected an RSA public key"),
    };

    // TODO: Test the fingerprint
}

#[test]
#[should_panic(expected = "Invalid format")]
fn test_rsa_pubkey_2048_invalid_format() {
    match sshkeys::PublicKey::from_path("tests/test-keys/id_rsa_2048_invalid_format.pub") {
        Ok(v)  => panic!("Expected invalid format, got {:?}", v),
        Err(e) => panic!("{}", e.description()),
    }
}

#[test]
#[should_panic(expected = "Unknown key type")]
fn test_rsa_pubkey_2048_unknown_keytype() {
    match sshkeys::PublicKey::from_path("tests/test-keys/id_rsa_2048_unknown_keytype.pub") {
        Ok(v)  => panic!("Expected unknown key type, got {:?}", v),
        Err(e) => panic!("{}", e.description()),
    }
}
