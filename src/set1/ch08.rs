use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn print() {
    let file = File::open(&"encrypted/ch08.txt").unwrap();
    let reader = BufReader::new(file);
    let mut line_ct = 0;
    for line in reader.lines() {
        line_ct += 1;
        let l = line.unwrap();
        
        //blocks of 16 bytes (32 hexstring chars)
        let blocks: Vec<String> = 
            (0..((l.len()/32)-1))
            .into_iter()
            .map(|i| l[(i*32)..((i+1)*32)]
            .to_string())
            .collect();

        //compare hexstring blocks
        for i in 0..blocks.len() {
            if (i+1..blocks.len())
                .into_iter()
                .any(|j| blocks[i] == blocks[j]) {
                    println!("Line {:?} is AES-128 encrypted",line_ct); 
                    return;
            }
        }
    }
}