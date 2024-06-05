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

//unused
use std::collections::HashMap;
use rand::{distributions::Uniform, Rng}; // 0.6.5

pub fn print() {
    //assign random key to global variable
    let random_key: [u8; 16] = rand::random();

    println!("{:?}", random_key);
}
