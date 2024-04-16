use crate::ch03::{decrypt_single_xor, best_key, apply_key};
use crate::ch05::apply_repeating_xor;
use base64_light::base64_decode;
use std::fs::File;
use std::io::{BufRead, BufReader};

//hamming distance is number of different bits
fn hamming_distance(b1: &[u8], b2: &[u8]) -> u32{
    b1.iter()
    .zip(b2.iter())
    .map(|(a,b)| (a^b).count_ones())
    .sum()
}


//score as hamming distance between first keysize bytes and second keysize bytes
//the more hamming sizes to calculate the better
fn score_keysize(encrypted: &[u8], keysize: usize) -> u32 {
    //i don't wanna hear anyone making fun of how i wrote this method, i promise i'll refactor it later
    let h1 = hamming_distance(&encrypted[0..keysize], &encrypted[keysize..(2*keysize)]);
    let h2 = hamming_distance(&encrypted[keysize..(2*keysize)], &encrypted[(2*keysize)..(3*keysize)]);
    let h3 = hamming_distance(&encrypted[(2*keysize)..(3*keysize)], &encrypted[(3*keysize)..(4*keysize)]);
    let h4 = hamming_distance(&encrypted[(3*keysize)..(4*keysize)], &encrypted[(4*keysize)..(5*keysize)]);
    let h5 = hamming_distance(&encrypted[(4*keysize)..(5*keysize)], &encrypted[(5*keysize)..(6*keysize)]);
    let h6 = hamming_distance(&encrypted[(5*keysize)..(6*keysize)], &encrypted[(6*keysize)..(7*keysize)]);
    let h7 = hamming_distance(&encrypted[(6*keysize)..(7*keysize)], &encrypted[(7*keysize)..(8*keysize)]);
    let h8 = hamming_distance(&encrypted[(7*keysize)..(8*keysize)], &encrypted[(8*keysize)..(9*keysize)]);
    let h9 = hamming_distance(&encrypted[(8*keysize)..(9*keysize)], &encrypted[(9*keysize)..(10*keysize)]);
    let h10 = hamming_distance(&encrypted[(9*keysize)..(10*keysize)], &encrypted[(10*keysize)..(11*keysize)]);
    let h11 = hamming_distance(&encrypted[(10*keysize)..(11*keysize)], &encrypted[(11*keysize)..(12*keysize)]);
    let h12 = hamming_distance(&encrypted[(11*keysize)..(12*keysize)], &encrypted[(12*keysize)..(13*keysize)]);
    h1 + h2 + h3 + h4 + h5 + h6 + h7 + h8 + h9 + h10 + h11 + h12
}

//best keysize minimizes keysize score
fn best_keysize(encrypted: &[u8]) -> usize{
    let max_keysize = (encrypted.len()/4).min(40);
    (2..=max_keysize)
    .into_iter()
    .min_by_key(|keysize| ((score_keysize(encrypted,*keysize)) as f32 * (1.0) / ((*keysize as f32).powf(1.0))) as u32) //minimize hamming distance
    .unwrap()
}


//every nth block
fn transposed_blocks(encrypted: &[u8], size: usize) -> Vec<Vec<u8>> {
    let mut transposed_blocks: Vec<Vec<u8>> = (0..size).map(|_| Vec::new()).collect();
    for block in encrypted.chunks(size) {
        for (&u, bt) in block.iter().zip(transposed_blocks.iter_mut()) {
            bt.push(u);
        }
    }
    transposed_blocks
}

//transpose and apply the 'single xor key' finder on all transposed parts
//apply collected key over whole encrypoted chain
fn decrypt_repeating_xor(encrypted: &[u8]) -> Vec<u8> {
    let keysize = best_keysize(&encrypted);
    let blocks = transposed_blocks(&encrypted, keysize);
    let encryption_key: Vec<u8> = blocks.iter().map(|block| best_key(&block)).collect();
    let decrypted = apply_repeating_xor(&encryption_key, encrypted);
    decrypted
}



pub fn print() {
    let mut content = String::new();
    let file = File::open(&"src/ch06.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        content.push_str(line.unwrap().trim());
    }
    let encrypted = base64_decode(&content);
    println!("\nch06:\n{}", String::from_utf8(decrypt_repeating_xor(&encrypted)).unwrap());
}
