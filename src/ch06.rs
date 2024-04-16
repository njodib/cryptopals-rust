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
    (0..=12)
    .into_iter()
    .map(|i| hamming_distance(&encrypted[(i*keysize)..((i+1)*keysize)], &encrypted[((i+1)*keysize)..((i+2)*keysize)]))
    .sum()
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
