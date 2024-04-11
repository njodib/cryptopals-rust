use crate::ch03::decrypt_single_xor;

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

fn distranspose_blocks(mut blocks: Vec<Vec<u8>>, message_size: u32, keysize: u32) -> Vec<u8> {
    //create our output Vector
    let mut output: Vec<u8> = Vec::new();

    //first find the block with the 'last letter'
    let last_block: u8 = match message_size % keysize {
        0 => (keysize-1) as u8,
        _ => ((message_size % keysize) - 0) as u8,
    };

    //start at the block with the last letters, and pop into the output until the remaining blocks are equally-sized
    for i in (0..last_block).rev() {
        output.push(blocks[i as usize].pop().unwrap());
    }

    //cycle down the vector, popping elements
    let num_elements_to_pop = message_size - (last_block as u32);
    let num_elements_in_block = num_elements_to_pop / keysize;

    for _ in 0..num_elements_in_block {
        for block in (0..keysize).rev() { //keysize = num blocks
            output.push(blocks[block as usize].pop().unwrap())
        }
    }

    //output the vector with popped elements in reverse
    let output_reverse: Vec<u8> = output.iter().copied().rev().collect();
    output_reverse
    
}


fn decrypt_message_for_keysize(encrypted: &[u8], keysize: usize) -> Vec<u8> {
    let mut solved_blocks: Vec<Vec<u8>> = Vec::new();
    for block in transposed_blocks(encrypted, keysize) {
        solved_blocks.push(decrypt_single_xor(&block));
    }
    distranspose_blocks(solved_blocks, encrypted.len() as u32, keysize as u32)
}

fn decrypt_repeating_xor(encrypted: &[u8]) -> Vec<u8> {
    let keysize = best_keysize(&encrypted);
    decrypt_message_for_keysize(&encrypted, keysize)
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
