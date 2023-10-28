/*
 * Copyright (c) TIKI Inc.
 * MIT license. See LICENSE file in root directory.
 */

use base16ct;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

pub fn sha256(input: &Vec<u8>) -> Vec<u8> {
    Sha256::new()
        .chain_update(input.as_slice())
        .finalize()
        .to_vec()
}

pub fn hmac_sha256(key: &Vec<u8>, input: &Vec<u8>) -> Vec<u8> {
    let hash = Hmac::<Sha256>::new_from_slice(key)
        .expect("Failed to resolve key")
        .chain_update(input.as_slice())
        .finalize();
    hash.into_bytes().to_vec()
}

pub fn to_hex(bytes: &Vec<u8>) -> String {
    base16ct::lower::encode_string(bytes.as_slice())
}

pub fn from_hex(input: &str) -> Vec<u8> {
    base16ct::lower::decode_vec(input).expect("Failed to decode input")
}

#[cfg(test)]
mod tests {
    use crate::utils::crypto::*;

    #[test]
    fn hash_success() {
        let test_string = "hello";
        let hash = sha256(&test_string.as_bytes().to_vec());
        assert_eq!(
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
            to_hex(&hash)
        )
    }

    #[test]
    fn hmac_success() {
        let test_key = "my secret and secure key";
        let test_string = "input message";
        let hash = hmac_sha256(
            &test_key.as_bytes().to_vec(),
            &test_string.as_bytes().to_vec(),
        );
        assert_eq!(
            "97d2a569059bbcd8ead4444ff99071f4c01d005bcefe0d3567e1be628e5fdcd9",
            to_hex(&hash)
        )
    }
}
