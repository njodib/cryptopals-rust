use crate::utils::{hex_decode};
use crate::xor::apply_xor_fixed;

pub fn print() {
    //from challenge 2
    let hex1 = "1c0111001f010100061a024b53535009181c";
    let hex2 = "686974207468652062756c6c277320657965";

    let b1 = hex_decode(&hex1);
    let b2 = hex_decode(&hex2);
    let bxor = apply_xor_fixed(&b1,&b2);

    println!("{}", String::from_utf8(bxor).unwrap());
}