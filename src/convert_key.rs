extern crate base64;

pub fn base64_to_bytes(base64_string: &str) -> Vec<u8> {
    base64::decode(base64_string).expect("Base64 decode failed")
}