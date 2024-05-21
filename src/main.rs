use std::env;

use challenges::set1;
use challenges::set2;
//use challenges::ch10;

fn main() {
    
    for ch in challenge_indices(12){
        println!("\nCHALLENGE {} PRINTOUT:", ch);
        match ch {
             1 => set1::ch01::print(),
             2 => set1::ch02::print(),
             3 => set1::ch03::print(),
             4 => set1::ch04::print(),
             5 => set1::ch05::print(),
             6 => set1::ch06::print(),
             7 => set1::ch07::print(),
             8 => set1::ch08::print(),
             9 => set2::ch09::print(),
            10 => set2::ch10::print(),
            11 => set2::ch11::print(),
            12 => set2::ch12::print(),
            _ => panic!("\nERROR: Tried to run challenge which either doesn't exist or hasn't been implemented.\n")
        }    
    }
}

fn challenge_indices(challenges_count: usize) -> Vec<usize>{
    //get the arguments from command line
    let args = env::args();

    //first arg is calling the program, handle case of no challenges specified
    if args.len() <= 1 {
        return (1..=challenges_count).collect();
    }

    //put args into 'indices' vector, if they exist
    let mut indices = Vec::new();
    for arg in args.skip(1) {
        if let Ok(index) = arg.parse::<usize>() {
            if index >= 1 && index <= challenges_count {
                indices.push(index);
                continue;
            }
        }
        panic!("\nERROR: Tried to run challenge which either doesn't exist or hasn't been implemented.\n")
    }

    //return full indices vector
    indices
}