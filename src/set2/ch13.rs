//ECB Cut and Paste

//1) Function: k=v parsing routine for a cookie. Por ejemplo: foo=bar&baz=qux&zap=zazzle -> object (JSON)
//2) Function: to make user. For example: 'profile_for("foo@bar.com")'. 
//          * Set fields of object as: 'email', 'uid', 'role'. Encode this as a structured cookie.
//          * Don't allow email to accept encoding metacharacters (& and =)
//3) Generate random AES key.
//4) Function: Encrypt encoded user profile under key. Provide this to attacker
//5) Function: Decrypt the encoded user profile and parse it
//6) Function: Create role=admin profile
//          * Generate valid ciphertexts from user_input to profile_for().
//          * This ciphertext will be the only thing you use to create user with role=admin

//used
use crate::aes_algs::aes_ecb_encrypt;
use rand::{distributions::Uniform, Rng}; // 0.6.5

//unused
use std::collections::HashMap;

//Generate random AES key
static random_key: [u8; 16] = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];


fn kv_decode<'a>(input: &[u8]) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut result = Vec::new();

    for kv in input.split(|&x| x == b'&') {
        let mut tmp = kv.split(|&x| x == b'=');
        match (tmp.next(), tmp.next()) {
            (Some(k), Some(v)) => {
                result.push((k.to_vec(), v.to_vec()));},
            _ => panic!("Invalid input"),
        }
    }
    result
}

pub fn profile_for(email: &str) -> Vec<u8> {
    let mut result: String = String::new();
    result.push_str(&"email=");
    result.push_str(&email.replace("&","").replace("=",""));
    result.push_str(&"&uid=10&role=user");
    result.as_bytes().to_vec()
}

pub fn create_encrypted_user(email: &str) -> Vec<u8> {
    let mut encoded_user = profile_for(email);
    aes_ecb_encrypt(&encoded_user, &random_key)
}


pub fn print() {

    let encrypted_user: Vec<u8> = create_encrypted_user("tilapia@hotmail.com");
    

    println!("{:?}", encrypted_user);
}
